import { Bot } from 'mineflayer';

import { getHold } from '../controllerApi';
import { Agent, MoveItemsOperationKind } from '../types';
import { sendChestData, transferItems } from './procedures';
import { openChestAt } from './procedures/openChestAt';

export const moveItems = async (
  operationKind: MoveItemsOperationKind,
  bot: Bot,
  agent: Agent
) => {
  const {
    data: {
      hold: { location: sourceLocation, slot: sourceSlot }
    }
  } = await getHold(operationKind.source_hold, agent);
  const {
    data: {
      hold: { location: destinationLocation, slot: destinationSlot }
    }
  } = await getHold(operationKind.destination_hold, agent);

  const sourceChest = await openChestAt(sourceLocation, bot, agent);

  await transferItems(
    bot,
    sourceChest,
    sourceSlot,
    0,
    operationKind.count,
    'from_chest'
  );

  await sendChestData(sourceChest, sourceLocation, agent);

  sourceChest.close();

  const destChest = await openChestAt(destinationLocation, bot, agent);

  await transferItems(
    bot,
    destChest,
    destinationSlot,
    0,
    operationKind.count,
    'to_chest'
  );

  await sendChestData(destChest, destinationLocation, agent);

  destChest.close();
};
