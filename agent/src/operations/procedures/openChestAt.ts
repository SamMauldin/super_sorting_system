import { Bot, Chest } from 'mineflayer';
import { Window } from 'prismarine-windows';
import vec3 from 'vec3';
import { navigateTo } from '.';

import { Agent, Location, Vec3 } from '../../types';

export const openChestAt = async (
  chestLoc: Location,
  openFrom: Vec3,
  bot: Bot,
  agent: Agent,
  skipNavigation?: boolean
): Promise<Chest & Window> => {
  if (!skipNavigation)
    await navigateTo(
      {
        dim: chestLoc.dim,
        vec3: openFrom
      },
      bot,
      agent
    );

  const chestBlock = bot.blockAt(vec3(chestLoc.vec3));
  if (!chestBlock) throw new Error('No block at chest destination!');

  // @ts-ignore mineflayer typing is wrong
  const chest: Chest & Window = await bot.openBlock(chestBlock);

  return chest;
};
