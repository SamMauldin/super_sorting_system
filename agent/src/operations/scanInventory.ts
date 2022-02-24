import { Bot } from "mineflayer";

import { Agent, ScanInventoryOperationKind } from "../types";
import { openChestAt, sendChestData } from "./procedures";

export const scanInventory = async (
  operationKind: ScanInventoryOperationKind,
  bot: Bot,
  agent: Agent
) => {
  const chest = await openChestAt(operationKind.location, bot, agent);

  await sendChestData(chest, operationKind.location, agent);

  chest.close();
};
