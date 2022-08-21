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

  const destinationSlots = await acquireFreeSpaces(27);

  try {
    await executeOperation(
      {
        type: 'ImportInventory',
        chest_location: destNode.pickup,
        node_location: destNode.location,
        destination_holds: destinationSlots,
      },
      'UserInteractive',
    );
  } finally {
    await releaseHolds(destinationSlots);
  }
};
