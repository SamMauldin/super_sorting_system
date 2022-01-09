import { config } from "dotenv";
config();

import { once } from "events";
import mineflayer from "mineflayer";
import { setTimeout } from "timers/promises";

import {
  heartbeat,
  operationComplete,
  pollOperation,
  registerAgent,
} from "./controllerApi";
import {
  dropItems,
  moveItems,
  scanInventory,
  importInventory,
} from "./operations";
import { navigateTo } from "./operations/procedures";
import { clearInventory, sleep } from "./utils";

const main = async () => {
  const {
    data: { agent, complex },
  } = await registerAgent();

  console.log(`Registered agent ${agent.id}`);

  setInterval(() => {
    heartbeat(agent).catch((err) => {
      console.error("Heartbeat failed", err);
      process.exit(1);
    });
  }, 1000 * 15);

  console.log("Creating mineflayer instance");

  const bot = mineflayer.createBot({
    host: process.env.AGENT_MC_SERVER!,
    username: process.env.AGENT_USERNAME!,
    password: process.env.AGENT_PASSWORD!,
  });

  bot.on("error", (err) => {
    console.error("Bot error", err);
    process.exit(1);
  });

  bot.on("kicked", (reason) => {
    console.error("Kicked", reason);
    process.exit(1);
  });

  await once(bot, "spawn");

  console.log("Received spawn event");

  await setTimeout(3000);

  const { x, y, z } = bot.player.entity.position;
  console.log(`Dimension: ${bot.game.dimension}, Location: (${x}, ${y}, ${z})`);

  while (true) {
    await clearInventory(bot, agent, complex);

    const { data: operationResponse } = await pollOperation(agent);

    if (operationResponse.type === "OperationAvailable") {
      const { operation } = operationResponse;

      if (operation.kind.type === "ScanInventory") {
        await scanInventory(operation.kind, bot, agent, complex);
      } else if (operation.kind.type === "MoveItems") {
        await moveItems(operation.kind, bot, agent, complex);
      } else if (operation.kind.type === "DropItems") {
        await dropItems(operation.kind, bot, agent, complex);
      } else if (operation.kind.type === "ImportInventory") {
        await importInventory(operation.kind, bot, agent, complex);
      } else {
        throw new Error("Unknown operation kind dispatched!");
      }

      console.log(`Completed ${operation.kind.type} Operation`);
      await operationComplete(agent, operation);
    } else {
      await navigateTo(
        { ...complex.bounds[0], y: complex.y_level + 1 },
        complex.dimension,
        bot,
        agent
      );
      await sleep(1000);
    }
  }
};

main()
  .then(() => {
    console.log("Exited");
    process.exit(0);
  })
  .catch((err) => {
    console.error("Exited with error", err);
    process.exit(1);
  });
