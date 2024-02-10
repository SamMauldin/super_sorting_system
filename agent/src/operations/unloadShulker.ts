import { Bot, Chest } from 'mineflayer';
import { Window } from 'prismarine-windows';
import { getHold } from '../controllerApi';
import vec3 from 'vec3';

import {
  Agent,
  locEq,
  Location,
  UnloadShulkerOperationKind,
  Vec3
} from '../types';
import {
  navigateTo,
  openChestAt,
  sendChestData,
  transferItems
} from './procedures';
import assert from 'assert';
import { setTimeout } from 'timers/promises';

export const unloadShulker = async (
  {
    shulker_station_location,
    shulker_hold,
    destination_holds
  }: UnloadShulkerOperationKind,
  bot: Bot,
  agent: Agent
) => {
  const {
    data: {
      hold: {
        location: shulkerChestLocation,
        slot: shulkerChestSlot,
        open_from: shulkerOpenFrom
      }
    }
  } = await getHold(shulker_hold, agent);

  // Grab Shulker
  let shulkerChest = await openChestAt(
    shulkerChestLocation,
    shulkerOpenFrom,
    bot,
    agent
  );
  await transferItems(bot, shulkerChest, shulkerChestSlot, 27, 1, 'from_chest');
  await sendChestData(
    shulkerChest,
    shulkerChestLocation,
    shulkerOpenFrom,
    agent
  );
  shulkerChest.close();

  // Place Shulker
  await navigateTo(shulker_station_location, bot, agent);
  bot.setQuickBarSlot(0);
  bot.updateHeldItem();
  assert(bot.inventory.slots[36]);
  assert(bot.inventory.slots[36].name.endsWith('shulker_box'));
  const piston = bot.blockAt(
    vec3(shulker_station_location.vec3).add(vec3([0, 3, 0]))
  );
  assert(piston);
  await bot.placeBlock(piston, vec3([0, -1, 0]));

  // Open Shulker
  const shulkerBlock = bot.blockAt(
    vec3(shulker_station_location.vec3).add(vec3([0, 2, 0]))
  );
  assert(shulkerBlock);

  // @ts-ignore mineflayer typing is wrong
  const shulker: Chest & Window = await bot.openBlock(shulkerBlock);

  const items = shulker.slots
    .slice(0, shulker.inventoryStart)
    .map((item, slot) => ({
      item,
      slot
    }));

  for (const { slot, item } of items) {
    if (!item) continue;
    if (slot >= destination_holds.length) continue;

    await transferItems(bot, shulker, slot, slot, Infinity, 'from_chest');
  }

  shulker.close();

  // Break Shulker
  const button = bot.blockAt(
    vec3(shulker_station_location.vec3).add(vec3([0, 4, 0]))
  );
  assert(button);
  await bot.activateBlock(button);

  // Wait for piston to fully retract
  await setTimeout(50 * 5);

  // Wait for shulker to be collected
  while (true) {
    await setTimeout(50);
    if (bot.inventory.slots[36]) break;
  }

  // Put Shulker back in Slot
  shulkerChest = await openChestAt(
    shulkerChestLocation,
    shulkerOpenFrom,
    bot,
    agent
  );
  await transferItems(bot, shulkerChest, shulkerChestSlot, 27, 1, 'to_chest');
  await sendChestData(
    shulkerChest,
    shulkerChestLocation,
    shulkerOpenFrom,
    agent
  );
  shulkerChest.close();

  // Transfer collected items to destination holds
  let lastChest: {
    location: Location;
    chest: Chest & Window;
    openFrom: Vec3;
  } | null = null;

  for (let i = 0; i < 27; i++) {
    if (!bot.inventory.slots[bot.inventory.inventoryStart + i]) continue;
    if (destination_holds.length <= i) continue;

    const {
      data: {
        hold: {
          location: destinationLocation,
          slot: destinationSlot,
          open_from: destOpenFrom
        }
      }
    } = await getHold(destination_holds[i], agent);

    if (lastChest && !locEq(destinationLocation, lastChest.location)) {
      await sendChestData(
        lastChest.chest,
        lastChest.location,
        lastChest.openFrom,
        agent
      );
      lastChest.chest.close();
      lastChest = null;
    }

    const destChest: Chest & Window =
      lastChest?.chest ||
      (await openChestAt(destinationLocation, destOpenFrom, bot, agent));

    await transferItems(
      bot,
      destChest,
      destinationSlot,
      i,
      Infinity,
      'to_chest'
    );

    lastChest = {
      chest: destChest,
      location: destinationLocation,
      openFrom: destOpenFrom
    };
  }

  if (lastChest) {
    await sendChestData(
      lastChest.chest,
      lastChest.location,
      lastChest.openFrom,
      agent
    );
    lastChest.chest.close();
  }
};
