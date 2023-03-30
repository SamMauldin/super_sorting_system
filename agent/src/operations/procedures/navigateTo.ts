import { Bot } from 'mineflayer';
import vec3, { Vec3 as depVec3 } from 'vec3';
import { findPath } from '../../controllerApi';
import { once } from 'events';
import { setTimeout } from 'timers/promises';
import timers from 'timers';

import { Agent, Vec3, vecEq, Location, stringToDim } from '../../types';

const floorVec3 = (input: Vec3) => ({
  x: Math.floor(input.x),
  y: Math.floor(input.y),
  z: Math.floor(input.z)
});

class FlyTimeoutError extends Error {
  constructor() {
    super('Timed out while flying');
    this.name = 'FlyTimeoutError';
  }
}

// Largely adapted from mineflayer

function vecMagnitude(vec: depVec3) {
  return Math.sqrt(vec.x * vec.x + vec.y * vec.y + vec.z * vec.z);
}

async function flyTo(bot: Bot, destination: depVec3) {
  const segmentLength = 5;

  let vector = destination.minus(bot.entity.position);
  let magnitude = vecMagnitude(vector);

  const allowedTravelTimeMs = 5000 + magnitude * 20;
  let travelTimeExceeded = false;
  let travelTimeTimeout = timers.setTimeout(() => {
    console.warn('NAV: Travel time exceeded! Attempting break');
    travelTimeExceeded = true;
  }, allowedTravelTimeMs);

  while (true) {
    let nextSegment = bot.entity.position;
    while (true) {
      const vecToDest = destination.minus(nextSegment);
      const distToEnd = vecMagnitude(vecToDest);
      const normalizedVector = vecToDest.scaled(1 / magnitude);

      const candidateSegment =
        distToEnd < segmentLength
          ? destination
          : nextSegment.add(normalizedVector.scaled(segmentLength));

      if (bot.world.getColumnAt(candidateSegment)) {
        nextSegment = candidateSegment;
        if (nextSegment.equals(destination)) break;
      } else {
        break;
      }
    }

    bot.entity.position = nextSegment;
    bot.entity.onGround = true;

    if (nextSegment.equals(destination)) {
      await once(bot, 'move');
      timers.clearTimeout(travelTimeTimeout);

      return;
    }

    await setTimeout(50);

    if (travelTimeExceeded) throw new FlyTimeoutError();
  }
}

const displayLoc = (loc: Location): string => {
  return `(${loc.vec3.x}, ${loc.vec3.y}, ${loc.vec3.z} in ${loc.dim})`;
};

const currentPosLoc = (bot: Bot): Location => {
  const { x, y, z } = bot.player.entity.position;
  const dim = stringToDim(bot.game.dimension);

  return {
    vec3: { x, y, z },
    dim
  };
};

export const takePortal = async (vec: Vec3, bot: Bot) => {
  const startingDim = bot.game.dimension;

  await setTimeout(700);
  await flyTo(bot, vec3(vec).add(vec3({ x: 0.5, y: 0, z: 0.5 })));

  console.log(
    `NAV: At portal location: ${displayLoc(
      currentPosLoc(bot)
    )}. Waiting for traversal.`
  );
  while (bot.game.dimension === startingDim) {
    await setTimeout(100);
  }
  console.log(
    `NAV: Portal traversed. New location: ${displayLoc(
      currentPosLoc(bot)
    )}. Awaiting chunk load.`
  );
  while (!bot.world.getColumnAt(bot.entity.position)) {
    await setTimeout(100);
  }
  console.log(`NAV: Chunk loaded. Continuing.`);
};

const navigateToImpl = async (
  destinationLoc: Location,
  bot: Bot,
  agent: Agent
): Promise<void> => {
  const { x, y, z } = bot.player.entity.position;
  const dim = stringToDim(bot.game.dimension);

  console.log(
    `NAV: Starting navigation from ${displayLoc(
      currentPosLoc(bot)
    )} to ${displayLoc(destinationLoc)}`
  );

  if (
    destinationLoc.dim === dim &&
    vecEq(floorVec3({ x, y, z }), floorVec3(destinationLoc.vec3))
  )
    return;

  const { data: pathResp } = await findPath(
    agent,
    { vec3: floorVec3({ x, y, z }), dim },
    destinationLoc
  );

  if (pathResp.type === 'Error') throw new Error('Pathfinding request failed!');

  for (const node of pathResp.path) {
    console.log('NAV: Navigating to next node', node);
    if ('Vec' in node) {
      await flyTo(bot, vec3(node.Vec).add(vec3({ x: 0.5, y: 0, z: 0.5 })));
    } else {
      await takePortal(node.Portal.vec, bot);
    }
  }

  console.log(`NAV: Finished`);
};

export const navigateTo = async (
  destinationLoc: Location,
  bot: Bot,
  agent: Agent
): Promise<void> => {
  const attempt = () => navigateToImpl(destinationLoc, bot, agent);
  let attemptCount = 0;

  while (attemptCount < 3) {
    attemptCount++;
    const res = await attempt()
      .then(() => true)
      .catch((err) => {
        console.warn('NAV: Attempt failed with ', err);
        return false;
      });

    if (res) return;
  }

  console.warn('NAV: All attempts failed');

  throw new Error('Navigation failed!');
};
