import { Bot, Chest } from 'mineflayer';
import { Window } from 'prismarine-windows';
import { getHold } from '../controllerApi';

import {
  Agent,
  ImportInventoryOperationKind,
  locEq,
  Location,
  Vec3
} from '../types';
import {
  navigateTo,
  openChestAt,
  sendChestData,
  transferItems
} from './procedures';

export const importInventory = async (
  {
    chest_location,
    node_location,
    destination_holds
  }: ImportInventoryOperationKind,
  bot: Bot,
  agent: Agent
) => {
  await navigateTo(node_location, bot, agent);

  const sourceChest = await openChestAt(
    { dim: node_location.dim, vec3: chest_location },
    node_location.vec3,
    bot,
    agent,
    true
  );

  const items = sourceChest.slots
    .slice(0, sourceChest.inventoryStart)
    .map((item, slot) => ({
      item,
      slot
    }))
    .filter(({ item }) => Boolean(item));

  let takenItemsCount = 0;

  for (const { slot } of items) {
    if (takenItemsCount === destination_holds.length) continue;

    await transferItems(
      bot,
      sourceChest,
      slot,
      takenItemsCount,
      Infinity,
      'from_chest'
    );
    takenItemsCount++;
  }

  sourceChest.close();

  let lastChest: {
    location: Location;
    chest: Chest & Window;
    openFrom: Vec3;
  } | null = null;

  for (let i = 0; i < takenItemsCount; i++) {
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
