import assert from "assert";
import { acquireFreeSpaces, executeOperation, releaseHolds } from "../helpers";
import { getPathfindingConfig } from "./automation";

export const pickupItems = async (destinationLoc: string): Promise<void> => {
  const {
    data: { nodes },
  } = await getPathfindingConfig();
  const destNode = nodes[destinationLoc];

  assert(destNode, "Destination location does not exist");
  assert(destNode.chest, "Destination does not have a chest");

  const destinationSlots = await acquireFreeSpaces(27);

  try {
    await executeOperation(
      {
        type: "ImportInventory",
        chest_location: destNode.chest,
        node_location: destNode.location,
        destination_holds: destinationSlots,
      },
      "UserInteractive"
    );
  } finally {
    await releaseHolds(destinationSlots);
  }
};
