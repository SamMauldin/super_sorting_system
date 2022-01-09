import { Bot, Chest } from "mineflayer";
import { Window } from "prismarine-windows";
import vec3 from "vec3";
import { navigateTo } from ".";

import { Agent, Vec3 } from "../../types";

export const openChestAt = async (
  chestLoc: Vec3,
  chestDim: string,
  bot: Bot,
  agent: Agent,
  skipNavigation?: boolean
): Promise<Chest & Window> => {
  if (!skipNavigation)
    await navigateTo({ ...chestLoc, y: chestLoc.y + 1 }, chestDim, bot, agent);

  const chestBlock = bot.blockAt(vec3(chestLoc));
  if (!chestBlock) throw new Error("No block at chest destination!");

  // @ts-ignore mineflayer typing is wrong
  const chest: Chest & Window = await bot.openChest(chestBlock);

  return chest;
};
