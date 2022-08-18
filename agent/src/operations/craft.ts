import { Bot, Chest } from 'mineflayer';
import { Window } from 'prismarine-windows';
import vec3 from 'vec3';

import { getHold } from '../controllerApi';
import { Agent, CraftOperationKind, Location, locEq } from '../types';
import { navigateTo, sendChestData, transferItems } from './procedures';
import { openChestAt } from './procedures/openChestAt';
import { craft as craftProc } from './procedures/craft';

export const craft = async (
  operationKind: CraftOperationKind,
  bot: Bot,
  agent: Agent
) => {
  let lastChest: { location: Location; chest: Chest & Window } | null = null;

  for (const [inv_slot, hold_id] of operationKind.recipe_source_holds.entries()) {
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

  await navigateTo(operationKind.node_location, bot, agent);

  const craftingTableBlock = bot.blockAt(vec3(operationKind.crafting_table_location));
  if (!craftingTableBlock) throw new Error('No block at crafting table destination!');

  // @ts-ignore mineflayer typing is wrong
  const craftingTable: Window = await bot.openBlock(craftingTableBlock);

  await craftProc(bot, craftingTable);

  bot.closeWindow(craftingTable);
};
