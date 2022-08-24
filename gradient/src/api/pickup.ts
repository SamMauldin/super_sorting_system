import assert from 'assert';
import { acquireFreeSpaces, executeOperation, releaseHolds } from '../helpers';
import { getSignConfig } from './automation';

export const pickupItems = async (destinationLoc: string): Promise<void> => {
  const {
    data: { nodes },
  } = await getSignConfig();
  const destNode = nodes[destinationLoc];

  assert(destNode, 'Destination location does not exist');
  assert(destNode.pickup, 'Destination does not have a pickup chest');

  const destinationSlotHoldIds = (await acquireFreeSpaces(27)).map(
    ({ id }) => id,
  );

  try {
    await executeOperation(
      {
        type: 'ImportInventory',
        chest_location: destNode.pickup,
        node_location: destNode.location,
        destination_holds: destinationSlotHoldIds,
      },
      'UserInteractive',
    );
  } finally {
    await releaseHolds(destinationSlotHoldIds);
  }
};
