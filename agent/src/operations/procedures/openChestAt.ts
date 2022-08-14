import { Bot, Chest } from 'mineflayer';
import { Window } from 'prismarine-windows';
import vec3 from 'vec3';
import { navigateTo } from '.';

import { Agent, Location } from '../../types';

export const openChestAt = async (
  chestLoc: Location,
  bot: Bot,
  agent: Agent,
  skipNavigation?: boolean
): Promise<Chest & Window> => {
  if (!skipNavigation)
    await navigateTo(
      {
        ...chestLoc,
        vec3: {
          ...chestLoc.vec3,
          y: chestLoc.vec3.y + 1
        }
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
