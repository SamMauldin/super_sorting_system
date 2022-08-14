import { Bot } from 'mineflayer';
import { getFreeHold } from './controllerApi';
import {
  openChestAt,
  sendChestData,
  transferItems
} from './operations/procedures';
import { Agent } from './types';

export const sleep = (delay: number) =>
  new Promise((resolve) => setTimeout(resolve, delay));

export const clearInventory = async (bot: Bot, agent: Agent) => {
  // Verify inventory empty
  for (const [invSlot, contents] of bot.inventory.slots.entries()) {
    if (!contents) continue;
    if (invSlot < bot.inventory.inventoryStart) continue;

    const { data } = await getFreeHold(agent);

    if (data.type !== 'HoldAcquired')
      throw new Error('Could not acquire a free hold to clear inventory!');

    const {
      hold: { location, slot }
    } = data;

    const chest = await openChestAt(location, bot, agent);

    await transferItems(
      bot,
      chest,
      slot,
      invSlot - bot.inventory.inventoryStart,
      Infinity,
      'to_chest'
    );

    await sendChestData(chest, location, agent);

    chest.close();
  }
};
