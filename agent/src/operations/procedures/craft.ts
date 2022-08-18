import { Bot } from 'mineflayer';
import itemLoader from 'prismarine-item';
import { Window } from 'prismarine-windows';
import { setTimeout } from 'timers/promises';

const Item = itemLoader('1.18.2');

export const craft = async (bot: Bot, crafting_table: Window) => {
  const invPacketSlot = 0 + crafting_table.inventoryStart;
  const invArrSlot = 0 + bot.inventory.inventoryStart;

  const craftingPacketSlot = 0;
  const craftingArrSlot = 0;

  const updatePlayerSlot = bot.inventory.updateSlot.bind(bot.inventory);
  const updateTableSlot = crafting_table.updateSlot.bind(crafting_table);

  // Move items into crafting table
  for (let i = 0; i < 9; i++) {
    const sourceArrSlot = invArrSlot + i;
    const sourcePacketSlot = invPacketSlot + i;

    const destArrSlot = craftingArrSlot + i + 1;
    const destPacketSlot = craftingPacketSlot + i + 1;

    const sourceItem = bot.inventory.slots[invArrSlot + i];
    if (!sourceItem) continue;

    await bot.clickWindow(sourcePacketSlot, 0, 0);
    // @ts-ignore Typings are wrong, this function accepts null for item
    updatePlayerSlot(sourceArrSlot, null);
    await bot.clickWindow(destPacketSlot, 0, 0);
    updateTableSlot(
      destArrSlot,
      new Item(
        sourceItem.type,
        sourceItem.count,
        sourceItem.metadata,
        // @ts-ignore Typings are wrong, this function accepts null for NBT
        sourceItem.nbt
      )
    );
  }

  await setTimeout(1000);

  console.log(crafting_table.slots);
};
