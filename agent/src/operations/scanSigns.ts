import { Bot } from 'mineflayer';

import { Agent, ScanSignsOperationKind } from '../types';
import { navigateTo, sendVisibleSignData, takePortal } from './procedures';

export const scanSigns = async (
  operationKind: ScanSignsOperationKind,
  bot: Bot,
  agent: Agent
) => {
  await navigateTo(operationKind.location, bot, agent);

  if (operationKind.take_portal) {
    await takePortal(operationKind.take_portal, bot);
  }

  await sendVisibleSignData(bot, agent);
};
