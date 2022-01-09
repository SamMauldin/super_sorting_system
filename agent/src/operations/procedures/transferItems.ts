import assert from "assert";
import { Bot, Chest } from "mineflayer";
import itemLoader from "prismarine-item";
import { Window } from "prismarine-windows";

const Item = itemLoader("1.17.1");

export const transferItems = async (
  bot: Bot,
  chest: Chest & Window,
  chestSlot: number,
  inventorySlot: number,
  count: number,
  direction: "to_chest" | "from_chest"
) => {
  const invPacketSlot = inventorySlot + chest.inventoryStart;
  const invArrSlot = inventorySlot + bot.inventory.inventoryStart;

  const chestPacketSlot = chestSlot;
  const chestArrSlot = chestSlot;

  const updateSourceSlot =
    direction === "to_chest"
      ? bot.inventory.updateSlot.bind(bot.inventory)
      : chest.updateSlot.bind(chest);
  const updateDestSlot =
    direction === "to_chest"
      ? chest.updateSlot.bind(chest)
      : bot.inventory.updateSlot.bind(bot.inventory);

  // Pickup items
  const sourceItem =
    direction === "to_chest"
      ? bot.inventory.slots[invArrSlot]
      : chest.slots[chestArrSlot];

  const sourcePacketSlot =
    direction === "to_chest" ? invPacketSlot : chestPacketSlot;
  const sourceArrSlot = direction === "to_chest" ? invArrSlot : chestArrSlot;

  assert(sourceItem);

  if (count === Infinity) {
    count = sourceItem.count;
  }
  assert(sourceItem.count >= count);

  const sourceItemCount = sourceItem.count;

  // Pickup whole stack
  await bot.clickWindow(sourcePacketSlot, 0, 0);

  // Drop until holding count === count
  let pickedUpCount = sourceItemCount;

  while (pickedUpCount > count) {
    await bot.clickWindow(sourcePacketSlot, 1, 0);
    pickedUpCount--;
  }

  if (count === sourceItemCount) {
    // @ts-ignore Typings are wrong, this function accepts null for item
    updateSourceSlot(sourceArrSlot, null);
  } else {
    updateSourceSlot(
      sourceArrSlot,
      new Item(
        sourceItem.type,
        sourceItemCount - count,
        sourceItem.metadata,
        // @ts-ignore Typings are wrong, this constructor accepts null for NBT
        sourceItem.nbt
      )
    );
  }

  // Move to destination
  const destinationItem =
    direction === "to_chest"
      ? chest.slots[chestArrSlot]
      : bot.inventory.slots[invArrSlot];
  const destinationItemCount = destinationItem?.count || 0;

  const destPacketSlot =
    direction === "to_chest" ? chestPacketSlot : invPacketSlot;
  const destArrSlot = direction === "to_chest" ? chestArrSlot : invArrSlot;

  await bot.clickWindow(destPacketSlot, 0, 0);

  updateDestSlot(
    destArrSlot,
    new Item(
      sourceItem.type,
      destinationItemCount + count,
      sourceItem.metadata,
      // @ts-ignore Typings are wrong, this constructor accepts null for NBT
      sourceItem.nbt
    )
  );
};
