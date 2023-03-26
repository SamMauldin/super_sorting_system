import assert from 'assert';
import { delay, releaseHolds } from '../helpers';
import {
  createHold,
  createOperation,
  getInventoryContents,
  getOperation,
  getSignConfig,
} from './automation';
import { HoldRequestFilter, StorageComplex } from './automation_types';
import { Loc, vec2ContainedBy } from './types';

const matchesComplex = (loc: Loc, complex: StorageComplex) => {
  if ('FlatFloor' in complex) {
    return (
      loc.dim === complex.FlatFloor.dimension &&
      loc.vec3.y === complex.FlatFloor.y_level &&
      vec2ContainedBy(complex.FlatFloor.bounds, loc.vec3)
    );
  } else if ('Tower' in complex) {
    const origin = complex.Tower.origin;
    return (
      loc.dim === complex.Tower.dimension &&
      loc.vec3.y >= origin.y &&
      loc.vec3.y < origin.y + complex.Tower.height &&
      vec2ContainedBy(
        [
          { x: origin.x - 4, z: origin.z - 4 },
          { x: origin.x + 4, z: origin.z + 4 },
        ],
        loc.vec3,
      )
    );
  } else {
    return false;
  }
};

export const complexTransfer = async (
  sourceComplexName: string,
  destinationComplexName: string,
): Promise<void> => {
  const {
    data: { complexes },
  } = await getSignConfig();
  const { data: inventories } = await getInventoryContents();
  const sourceComplex = complexes[sourceComplexName];
  const destinationComplex = complexes[destinationComplexName];

  assert(sourceComplex, 'Source complex does not exist');
  assert(destinationComplex, 'Destination complex does not exist');

  const sourceHoldRequests: HoldRequestFilter[] = [];
  const destinationHoldRequests: HoldRequestFilter[] = [];

  for (const inv of inventories) {
    if (matchesComplex(inv.loc, sourceComplex)) {
      for (const [slot, item] of inv.slots.entries()) {
        if (!item) continue;
        sourceHoldRequests.push({
          SlotLocation: { location: inv.loc, slot, open_from: inv.open_from },
        });
      }
    }

    if (matchesComplex(inv.loc, destinationComplex)) {
      for (const [slot, item] of inv.slots.entries()) {
        if (item) continue;
        destinationHoldRequests.push({
          SlotLocation: { location: inv.loc, slot, open_from: inv.open_from },
        });
      }
    }
  }

  const sourceHoldRequestResults = await createHold(sourceHoldRequests);
  const sourceHolds = [];

  for (const holdRes of sourceHoldRequestResults.data.results) {
    if ('Error' in holdRes) continue;
    sourceHolds.push(...holdRes.Holds.holds.map(({ id }) => id));
  }

  if (sourceHolds.length < destinationHoldRequests.length) {
    destinationHoldRequests.length = sourceHolds.length;
  }

  const destinationHoldRequestResults = await createHold(
    destinationHoldRequests,
  );
  const destinationHolds = [];

  for (const holdRes of destinationHoldRequestResults.data.results) {
    if ('Error' in holdRes) continue;
    destinationHolds.push(...holdRes.Holds.holds.map(({ id }) => id));
  }

  const operationIds = [];

  for (
    let i = 0;
    i < Math.min(destinationHolds.length, sourceHolds.length);
    i += 27
  ) {
    const {
      data: { operation },
    } = await createOperation(
      {
        type: 'MoveItems',
        source_holds: sourceHolds.slice(i, i + 27),
        destination_holds: destinationHolds.slice(i, i + 27),
        counts: Array(27).fill(-1),
      },
      'UserInteractive',
    );

    operationIds.push(operation.id);
  }

  let failed = false;

  while (operationIds.length > 0) {
    const {
      data: {
        operation: { status },
      },
    } = await getOperation(operationIds[operationIds.length - 1]);

    if (status === 'Complete') {
      operationIds.pop();
      continue;
    } else if (status === 'Aborted') {
      operationIds.pop();
      failed = true;
      continue;
    }

    await delay(5000);
  }

  await releaseHolds([...destinationHolds, ...sourceHolds]);

  if (failed) {
    throw new Error('One or more operations failed');
  }
};
