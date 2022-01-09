import { Bot, Chest } from "mineflayer";
import { Window } from "prismarine-windows";
import vec3 from "vec3";

import { getHold } from "../controllerApi";
import {
  Agent,
  ComplexInfo,
  DropItemsOperationKind,
  Vec3,
  vecEq,
} from "../types";
import {
  dropSlot,
  navigateTo,
  openChestAt,
  sendChestData,
  transferItems,
} from "./procedures";

export const dropItems = async (
  operationKind: DropItemsOperationKind,
  bot: Bot,
  agent: Agent,
  complex: ComplexInfo
) => {
  let lastChest: { location: Vec3; chest: Chest & Window } | null = null;

  for (const [inv_slot, hold_id] of operationKind.source_holds.entries()) {
    const {
      data: {
        hold: { location: sourceLocation, slot: sourceSlot },
      },
    } = await getHold(hold_id, agent);

    if (lastChest && !vecEq(sourceLocation, lastChest.location)) {
      await sendChestData(lastChest.chest, lastChest.location, agent);
      lastChest.chest.close();
      lastChest = null;
    }

    const chest: Chest & Window =
      lastChest?.chest ||
      (await openChestAt(sourceLocation, complex.dimension, bot, agent));

    await transferItems(
      bot,
      chest,
      sourceSlot,
      inv_slot,
      Infinity,
      "from_chest"
    );

    lastChest = { chest, location: sourceLocation };
  }

  if (lastChest) {
    await sendChestData(lastChest.chest, lastChest.location, agent);
    lastChest.chest.close();
  }

  await navigateTo(operationKind.drop_from, complex.dimension, bot, agent);

  await bot.lookAt(vec3(operationKind.aim_towards));

  for (const inv_slot of operationKind.source_holds.keys()) {
    await dropSlot(bot, inv_slot);
  }
};
