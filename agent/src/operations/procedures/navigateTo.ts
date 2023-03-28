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
  const normalizedVector = vector.scaled(1 / magnitude);

  const allowedTravelTimeMs = 5000 + magnitude * 10;
  let travelTimeExceeded = false;
  let travelTimeTimeout = timers.setTimeout(() => {
    console.log('Travel time exceeded! Attempting break');
    travelTimeExceeded = true;
  }, allowedTravelTimeMs);

  while (true) {
    let nextSegment = bot.entity.position;
    while (true) {
      const distToEnd = vecMagnitude(destination.minus(nextSegment));

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

export const takePortal = async (vec: Vec3, bot: Bot) => {
  const startingDim = bot.game.dimension;

  await setTimeout(700);
  await flyTo(bot, vec3(vec).add(vec3({ x: 0.5, y: 0, z: 0.5 })));

  console.log('At portal, awaiting new dimension...');
  while (bot.game.dimension === startingDim) {
    await setTimeout(100);
  }
  console.log('Portal traversed.');
};

const navigateToImpl = async (
  destinationLoc: Location,
  bot: Bot,
  agent: Agent
): Promise<void> => {
  const { x, y, z } = bot.player.entity.position;
  const dim = stringToDim(bot.game.dimension);

  console.log('=== Starting Navigation ===');
  console.log('Starting ', { x, y, z }, dim);

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
    console.log('Navigating to next node', node);
    if ('Vec' in node) {
      await flyTo(bot, vec3(node.Vec).add(vec3({ x: 0.5, y: 0, z: 0.5 })));
    } else {
      await takePortal(node.Portal.vec, bot);
    }
  }

  console.log('=== Navigation Finished ===');
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
        console.warn('Navigation attempt failed with ', err);
        return false;
      });

    if (res) return;
  }

  throw new Error('Navigation failed!');
};
