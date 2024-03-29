import { Bot } from 'mineflayer';
import { getFreeHold, releaseHold } from './controllerApi';
import {
  openChestAt,
  sendChestData,
  transferItems
} from './operations/procedures';
import { Agent } from './types';

export const sleep = (delay: number) =>
  new Promise((resolve) => setTimeout(resolve, delay));

export const clearInventory = async (
  bot: Bot,
  agent: Agent
): Promise<boolean> => {
  // Verify inventory empty
  for (const [invSlot, contents] of bot.inventory.slots.entries()) {
    if (!contents) continue;
    if (
      invSlot < bot.inventory.inventoryStart ||
      invSlot > bot.inventory.inventoryEnd
    )
      continue;

    const { data } = await getFreeHold(agent);

    if (data.type !== 'HoldAcquired') {
      console.error('Could not acquire a free hold to clear inventory!');
      return false;
    }

    const {
      hold: { location, slot, id, open_from }
    } = data;

    const chest = await openChestAt(location, open_from, bot, agent);

    await transferItems(
      bot,
      chest,
      slot,
      invSlot - bot.inventory.inventoryStart,
      Infinity,
      'to_chest'
    );

    await sendChestData(chest, location, open_from, agent);

    chest.close();

    await releaseHold(id);
  }

  return true;
};
