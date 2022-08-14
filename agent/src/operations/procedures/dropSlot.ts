import assert from 'assert';
import { Bot } from 'mineflayer';

export const dropSlot = async (bot: Bot, inventorySlot: number) => {
  const invArrSlot = inventorySlot + bot.inventory.inventoryStart;
  const invPacketSlot = invArrSlot;

  const sourceItem = bot.inventory.slots[invArrSlot];

  assert(sourceItem);

  await bot.clickWindow(invPacketSlot, 0, 0);

  // @ts-ignore Mineflayer typings are wrong
  bot.inventory.updateSlot(invArrSlot, null);

  await bot.clickWindow(-999, 0, 0);
};
