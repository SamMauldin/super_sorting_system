import { Bot } from "mineflayer";

import { Agent, ComplexInfo, ScanInventoryOperationKind } from "../types";
import { openChestAt, sendChestData } from "./procedures";

export const scanInventory = async (
  operationKind: ScanInventoryOperationKind,
  bot: Bot,
  agent: Agent,
  complex: ComplexInfo
) => {
  const chest = await openChestAt(
    operationKind.location,
    complex.dimension,
    bot,
    agent
  );

  await sendChestData(chest, operationKind.location, agent);

  chest.close();
};
