import { Bot, Chest } from 'mineflayer';
import { Window } from 'prismarine-windows';
import vec3 from 'vec3';

import { getHold } from '../controllerApi';
import { Agent, DropItemsOperationKind, Location, locEq, Vec3 } from '../types';
import {
  dropSlot,
  navigateTo,
  openChestAt,
  sendChestData,
  transferItems
} from './procedures';

export const dropItems = async (
  operationKind: DropItemsOperationKind,
  bot: Bot,
  agent: Agent
) => {
  let lastChest: {
    location: Location;
    chest: Chest & Window;
    openFrom: Vec3;
  } | null = null;

  for (const [inv_slot, hold_id] of operationKind.source_holds.entries()) {
    const {
      data: {
        hold: { location: sourceLocation, slot: sourceSlot, open_from }
      }
    } = await getHold(hold_id, agent);

    if (lastChest && !locEq(sourceLocation, lastChest.location)) {
      await sendChestData(
        lastChest.chest,
        lastChest.location,
        lastChest.openFrom,
        agent
      );
      lastChest.chest.close();
      lastChest = null;
    }

    const chest: Chest & Window =
      lastChest?.chest ||
      (await openChestAt(sourceLocation, open_from, bot, agent));

    await transferItems(
      bot,
      chest,
      sourceSlot,
      inv_slot,
      Infinity,
      'from_chest'
    );

    lastChest = { chest, location: sourceLocation, openFrom: open_from };
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

  await navigateTo(operationKind.drop_from, bot, agent);

  await bot.lookAt(vec3(operationKind.aim_towards));

  for (const inv_slot of operationKind.source_holds.keys()) {
    await dropSlot(bot, inv_slot);
  }
};
