import { Bot } from "mineflayer";

import { Agent, ScanSignsOperationKind } from "../types";
import { navigateTo, sendVisibleSignData } from "./procedures";

export const scanSigns = async (
  operationKind: ScanSignsOperationKind,
  bot: Bot,
  agent: Agent
) => {
  await navigateTo(operationKind.location, bot, agent);

  await sendVisibleSignData(bot, agent);
};
