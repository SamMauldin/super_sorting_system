import assert from 'assert';
import { acquireFreeSpaces, executeOperation, releaseHolds } from '../helpers';
import { getInventoryContents, getSignConfig } from './automation';
import { locEq } from './types';

export const pickupItems = async (
  destinationLoc: string,
  repeatUntilEmpty?: boolean,
): Promise<void> => {
  const {
    data: { nodes },
  } = await getSignConfig();
  const destNode = nodes[destinationLoc];

  assert(destNode, 'Destination location does not exist');
  assert(destNode.pickup, 'Destination does not have a pickup chest');

  let allSlotsFree = true;

  do {
    const destinationSlotHolds = await acquireFreeSpaces(27);
    const destinationSlotHoldIds = destinationSlotHolds.map(({ id }) => id);

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
      if (repeatUntilEmpty) {
        let hadItems = false;

        const contents = (await getInventoryContents()).data;
        for (const inventory of contents) {
          for (const hold of destinationSlotHolds) {
            if (locEq(inventory.loc, hold.location)) {
              if (inventory.slots[hold.slot]) {
                hadItems = true;
              }
            }
          }
        }

        allSlotsFree = !hadItems;
      }
      await releaseHolds(destinationSlotHoldIds);
    }
  } while (repeatUntilEmpty && !allSlotsFree);
};
