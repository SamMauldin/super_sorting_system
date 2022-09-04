import { Bot, Chest } from 'mineflayer';
import { Window } from 'prismarine-windows';
import { getHold } from '../controllerApi';
import vec3 from 'vec3';

import { Agent, locEq, Location, LoadShulkerOperationKind } from '../types';
import {
  navigateTo,
  openChestAt,
  sendChestData,
  transferItems
} from './procedures';
import assert from 'assert';
import { setTimeout } from 'timers/promises';

export const loadShulker = async (
  {
    shulker_station_location,
    shulker_hold,
    source_holds
  }: LoadShulkerOperationKind,
  bot: Bot,
  agent: Agent
) => {
  const {
    data: {
      hold: { location: shulkerChestLocation, slot: shulkerChestSlot }
    }
  } = await getHold(shulker_hold, agent);

  // Grab Shulker
  let shulkerChest = await openChestAt(shulkerChestLocation, bot, agent);
  await transferItems(bot, shulkerChest, shulkerChestSlot, 27, 1, 'from_chest');
  await sendChestData(shulkerChest, shulkerChestLocation, agent);
  shulkerChest.close();

  // Grab Items to load
  let lastChest: { location: Location; chest: Chest & Window } | null = null;

  for (const [inv_slot, hold_id] of source_holds.entries()) {
    if (!hold_id) continue;
    const {
      data: {
        hold: { location: sourceLocation, slot: sourceSlot }
      }
    } = await getHold(hold_id, agent);

    if (lastChest && !locEq(sourceLocation, lastChest.location)) {
      await sendChestData(lastChest.chest, lastChest.location, agent);
      lastChest.chest.close();
      lastChest = null;
    }

    const chest: Chest & Window =
      lastChest?.chest || (await openChestAt(sourceLocation, bot, agent));

    await transferItems(
      bot,
      chest,
      sourceSlot,
      inv_slot,
      Infinity,
      'from_chest'
    );

    lastChest = { chest, location: sourceLocation };
  }

  if (lastChest) {
    await sendChestData(lastChest.chest, lastChest.location, agent);
    lastChest.chest.close();
  }

  // Place Shulker
  await navigateTo(shulker_station_location, bot, agent);
  bot.setQuickBarSlot(0);
  bot.updateHeldItem();
  const piston = bot.blockAt(
    vec3(shulker_station_location.vec3).add(vec3([0, 4, 0]))
  );
  assert(piston);
  await bot.placeBlock(piston, vec3([0, -1, 0]));

  // Open Shulker
  const shulkerBlock = bot.blockAt(
    vec3(shulker_station_location.vec3).add(vec3([0, 3, 0]))
  );
  assert(shulkerBlock);

  // @ts-ignore mineflayer typing is wrong
  const shulker: Chest & Window = await bot.openBlock(shulkerBlock);

  for (let i = 0; i < 27; i++) {
    if (!bot.inventory.slots[bot.inventory.inventoryStart + i]) continue;

    await transferItems(bot, shulker, i, i, Infinity, 'to_chest');
  }

  shulker.close();

  // Break Shulker
  const button = bot.blockAt(
    vec3(shulker_station_location.vec3).add(vec3([0, 5, 0]))
  );
  assert(button);
  await bot.activateBlock(button);

  // Wait for shulker to be collected
  while (true) {
    await setTimeout(200);
    if (bot.inventory.slots[36]) break;
  }

  // Put Shulker back in Slot
  shulkerChest = await openChestAt(shulkerChestLocation, bot, agent);
  await transferItems(bot, shulkerChest, shulkerChestSlot, 27, 1, 'to_chest');
  await sendChestData(shulkerChest, shulkerChestLocation, agent);
  shulkerChest.close();
};
