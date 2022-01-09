import assert from "assert";
import {
  acquireFreeSpaces,
  acquireHoldVerified,
  releaseHolds,
  renewHolds,
  searchFor,
  ExtendedItem,
  executeOperation,
} from "../helpers";
import { getPathfindingConfig } from "./automation";

export const deliverItems = async (
  destinationLoc: string,
  itemList: {
    item: ExtendedItem;
    count: number;
  }[]
): Promise<void> => {
  const {
    data: { nodes },
  } = await getPathfindingConfig();
  const destNode = nodes[destinationLoc];

  assert(destNode, "Destination location does not exist");
  assert(destNode.portal, "Destination does not have portal");

  const slotsToDeliver: string[] = [];
  let tempHold: string | null = null;

  const renewInterval = setInterval(() => {
    renewHolds(slotsToDeliver).catch(() => {});
  }, 1000 * 60 * 2);

  try {
    // Collect slots for every type of item
    for (const item of itemList) {
      // Collect all items neccesary
      const slotsNeeded = Math.ceil(item.count / item.item.stack_size);
      for (let i = 0; i < slotsNeeded; i++) {
        const countDesired =
          i === slotsNeeded - 1
            ? item.count - item.item.stack_size * (slotsNeeded - 1)
            : item.item.stack_size;

        let countAcquired = 0;
        let filledSpace = null;

        // Fill up this slot
        while (countAcquired < countDesired) {
          const toMove = await searchFor(item.item, true);

          if (!filledSpace) {
            const exactMatch = toMove.find(
              ({ contents }) => contents!.count === countDesired
            );

            if (exactMatch) {
              const exactMatchHold = await acquireHoldVerified(
                exactMatch.loc,
                exactMatch.slot,
                exactMatch.contents
              ).catch(() => null);

              if (!exactMatchHold) continue;

              slotsToDeliver.push(exactMatchHold);

              break;
            }
          }

          if (!toMove[0]) throw new Error("Could not acquire item");

          const { loc, slot, contents } = toMove[0];

          const toMoveHold = await acquireHoldVerified(
            loc,
            slot,
            contents
          ).catch(() => null);
          tempHold = toMoveHold;
          if (!toMoveHold) continue;
          const toMoveCount = Math.min(
            countDesired - countAcquired,
            contents!.count
          );

          if (!filledSpace) {
            [filledSpace] = await acquireFreeSpaces(1);

            slotsToDeliver.push(filledSpace);
          }

          await executeOperation(
            {
              type: "MoveItems",
              source_hold: toMoveHold,
              destination_hold: filledSpace,
              count: toMoveCount,
            },
            "UserInteractive"
          );

          await releaseHolds([toMoveHold]);

          tempHold = null;
          countAcquired += toMoveCount;
        }
      }
    }

    // TODO Chunk deliveries
    assert(slotsToDeliver.length <= 27, "Too many slots to deliver!");

    await executeOperation(
      {
        type: "DropItems",
        source_holds: slotsToDeliver,
        drop_from: destNode.location,
        aim_towards: destNode.portal!.location,
      },
      "UserInteractive"
    );
  } finally {
    clearInterval(renewInterval);
    await releaseHolds(slotsToDeliver).catch(() => null);
    if (tempHold) await releaseHolds([tempHold]).catch(() => null);
  }
};
