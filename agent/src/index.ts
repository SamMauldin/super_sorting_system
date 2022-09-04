import { config } from 'dotenv';
config();

import { once } from 'events';
import mineflayer from 'mineflayer';
import { setTimeout } from 'timers/promises';

import {
  getSignConfig,
  heartbeat,
  operationComplete,
  pollOperation,
  registerAgent
} from './controllerApi';
import {
  dropItems,
  moveItems,
  scanInventory,
  importInventory,
  scanSigns,
  unloadShulker,
  loadShulker,
} from './operations';
import { navigateTo, sendVisibleSignData } from './operations/procedures';
import { clearInventory, sleep } from './utils';

const main = async () => {
  const {
    data: { agent }
  } = await registerAgent();

  console.log(`Registered agent ${agent.id}`);

  setInterval(() => {
    heartbeat(agent).catch((err) => {
      console.error('Heartbeat failed', err);
      process.exit(1);
    });
  }, 1000 * 15);

  console.log('Creating mineflayer instance');

  const bot = mineflayer.createBot({
    host: process.env.AGENT_MC_SERVER_HOST!,
    port: process.env.AGENT_MC_SERVER_PORT
      ? Number(process.env.AGENT_MC_SERVER_PORT)
      : undefined,
    username: process.env.AGENT_USERNAME!,
    auth: 'microsoft',
    version: '1.18.2'
  });

  bot.on('error', (err) => {
    console.error('Bot error', err);
    process.exit(1);
  });

  bot.on('kicked', (reason) => {
    console.error('Kicked', reason);
    process.exit(1);
  });

  await once(bot, 'spawn');

  console.log('Received spawn event');

  await setTimeout(3000);

  const { x, y, z } = bot.player.entity.position;
  console.log(`Dimension: ${bot.game.dimension}, Location: (${x}, ${y}, ${z})`);

  await sendVisibleSignData(bot, agent);

  let atHome = false;

  while (true) {
    await clearInventory(bot, agent);

    const { data: operationResponse } = await pollOperation(agent);

    if (operationResponse.type === 'OperationAvailable') {
      atHome = false;
      const { operation } = operationResponse;

      console.log(`Starting ${operation.kind.type} operation`);

      try {
        if (operation.kind.type === 'ScanInventory') {
          await scanInventory(operation.kind, bot, agent);
        } else if (operation.kind.type === 'MoveItems') {
          await moveItems(operation.kind, bot, agent);
        } else if (operation.kind.type === 'DropItems') {
          await dropItems(operation.kind, bot, agent);
        } else if (operation.kind.type === 'ImportInventory') {
          await importInventory(operation.kind, bot, agent);
        } else if (operation.kind.type === 'ScanSigns') {
          await scanSigns(operation.kind, bot, agent);
        } else if (operation.kind.type === 'UnloadShulker') {
          await unloadShulker(operation.kind, bot, agent);
	} else if (operation.kind.type === 'LoadShulker') {
	  await loadShulker(operation.kind, bot, agent);
        } else {
          throw new Error('Unknown operation kind dispatched!');
        }

        console.log(`Completed ${operation.kind.type} Operation`);
        await operationComplete(agent, operation, 'Complete');
      } catch (e) {
        console.error(e);
        console.log('Error while attempting operation!');
        await operationComplete(agent, operation, 'Aborted');
      }
    } else {
      if (!atHome) {
        const {
          data: { complexes }
        } = await getSignConfig();
        const complex = Object.values(complexes)[0];

        if (complex) {
          await navigateTo(
            {
              dim: complex.dimension,
              vec3: {
                ...complex.bounds[0],
                y: complex.y_level + 1
              }
            },
            bot,
            agent
          );
        }

        atHome = true;
      }
      await sleep(1000);
    }
  }
};

main()
  .then(() => {
    console.log('Exited');
    process.exit(0);
  })
  .catch((err) => {
    console.error('Exited with error', err);
    process.exit(1);
  });
