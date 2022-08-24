import assert from 'assert';
import {
  acquireFreeSpaces,
  releaseHolds,
  renewHolds,
  ExtendedItem,
  executeOperation,
} from '../helpers';
import { createHold, getSignConfig } from './automation';
import { HoldRequestFilter } from './automation_types';

export const deliverItems = async (
  destinationLoc: string,
  itemList: {
    item: ExtendedItem;
    count: number;
  }[],
): Promise<void> => {
  const {
    data: { nodes },
  } = await getSignConfig();
  const destNode = nodes[destinationLoc];

  assert(destNode, 'Destination location does not exist');
  assert(destNode.dropoff, 'Destination does not have a drop-off location');

  const holdsToDeliver: string[] = [];

  try {
    const itemRequests: HoldRequestFilter[] = itemList.map(
      ({ item, count }) => ({
        ItemMatch: {
          match_criteria: {
            StackableHash: { stackable_hash: item.stackable_hash },
          },
          total: count,
        },
      }),
    );
    const holdRequestResults = await createHold(itemRequests);

    for (const holdRes of holdRequestResults.data.results) {
      if ('Error' in holdRes) {
        throw new Error('Failed to acquire items');
      }

      holdsToDeliver.push(...holdRes.Holds.holds.map(({ id }) => id));
    }

    // TODO Chunk deliveries
    assert(holdsToDeliver.length <= 27, 'Too many slots to deliver!');

    await executeOperation(
      {
        type: 'DropItems',
        source_holds: holdsToDeliver,
        drop_from: destNode.location,
        aim_towards: destNode.dropoff,
      },
      'UserInteractive',
    );
  } finally {
    await releaseHolds(holdsToDeliver).catch(() => null);
  }
};
