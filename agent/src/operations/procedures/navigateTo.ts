import { Bot } from "mineflayer";
import vec3, { Vec3 as depVec3 } from "vec3";
import { findPath } from "../../controllerApi";
import { once } from "events";
import { setTimeout } from "timers/promises";

import { Agent, Vec3, vecEq, Location, stringToDim } from "../../types";

const floorVec3 = (input: Vec3) => ({
  x: Math.floor(input.x),
  y: Math.floor(input.y),
  z: Math.floor(input.z),
});

// Largely adapted from mineflayer

function vecMagnitude(vec: depVec3) {
  return Math.sqrt(vec.x * vec.x + vec.y * vec.y + vec.z * vec.z);
}

async function flyTo(bot: Bot, destination: depVec3) {
  const flyingSpeedPerUpdate = 5;
  const flyingSpeedEaseOut = 0.5;

  bot.creative.startFlying();

  let vector = destination.minus(bot.entity.position);
  let magnitude = vecMagnitude(vector);

  while (magnitude > flyingSpeedPerUpdate) {
    bot.physics.gravity = 0;
    bot.entity.velocity = vec3([0, 0, 0]);

    const normalizedVector = vector.scaled(1 / magnitude);
    bot.entity.position.add(normalizedVector.scaled(flyingSpeedPerUpdate));

    await setTimeout(50);

    vector = destination.minus(bot.entity.position);
    magnitude = vecMagnitude(vector);
  }

  while (magnitude > flyingSpeedEaseOut) {
    bot.physics.gravity = 0;
    bot.entity.velocity = vec3([0, 0, 0]);

    const normalizedVector = vector.scaled(1 / magnitude);
    bot.entity.position.add(normalizedVector.scaled(flyingSpeedEaseOut));

    await once(bot, "move");

    vector = destination.minus(bot.entity.position);
    magnitude = vecMagnitude(vector);
  }

  // last step
  bot.entity.position = destination;
  await once(bot, "move");
}

export const navigateTo = async (
  destinationLoc: Location,
  bot: Bot,
  agent: Agent
): Promise<void> => {
  const { x, y, z } = bot.player.entity.position;
  const dim = stringToDim(bot.game.dimension);

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

  if (pathResp.type === "Error") throw new Error("Pathfinding request failed!");

  for (const node of pathResp.path) {
    await flyTo(bot, vec3(node).add(vec3({ x: 0.5, y: 0, z: 0.5 })));
  }
};
