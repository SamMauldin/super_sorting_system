import { Bot } from 'mineflayer';

import { Agent, ScanInventoryOperationKind } from '../types';
import { openChestAt, sendChestData } from './procedures';

export const scanInventory = async (
  operationKind: ScanInventoryOperationKind,
  bot: Bot,
  agent: Agent
) => {
  const chest = await openChestAt(
    operationKind.location,
    operationKind.open_from,
    bot,
    agent
  );

  await sendChestData(
    chest,
    operationKind.location,
    operationKind.open_from,
    agent
  );

  chest.close();
};
