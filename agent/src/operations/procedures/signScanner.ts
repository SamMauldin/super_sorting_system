import { Bot } from 'mineflayer';
import { Agent, stringToDim, Vec3 } from '../../types';
import vec3 from 'vec3';
import { ScanRegion, sendSignScanData, Sign } from '../../controllerApi';
import { setTimeout } from 'timers';

const CHUNK_LOAD_DEBOUNCE_TIMER = 2000;

export const waitChunksRoughlyFinishedLoading = (bot: Bot): Promise<void> => {
  return new Promise<void>((resolve) => {
    let currTimeout: NodeJS.Timeout;
    const handler = () => {
      clearTimeout(currTimeout);

      setTimeout(finish, CHUNK_LOAD_DEBOUNCE_TIMER);
    };

    const finish = () => {
      bot.world.off('chunkColumnLoad', handler);
      resolve();
    };

    bot.world.on('chunkColumnLoad', handler);

    currTimeout = setTimeout(finish, CHUNK_LOAD_DEBOUNCE_TIMER);
  });
};

export const sendVisibleSignData = async (bot: Bot, agent: Agent) => {
  await waitChunksRoughlyFinishedLoading(bot);
  const scanRegions = await scanVisibleRegion(bot);

  await sendSignScanData(agent, scanRegions);
};

const yieldTick = () => new Promise((resolve) => setImmediate(resolve));

const scanVisibleRegion = async (bot: Bot): Promise<ScanRegion[]> => {
  const loadedChunks = bot.world.getColumns();

  const scannedRegions: ScanRegion[] = [];

  for (const chunk of loadedChunks) {
    const chunkPos = {
      x: parseInt(chunk.chunkX) * 16,
      y: 0,
      z: parseInt(chunk.chunkZ) * 16
    };

    const scanRegion = getSignsInChunk(bot, chunkPos, chunk);
    if (scanRegion) scannedRegions.push(scanRegion);

    await yieldTick();
  }

  return scannedRegions;
};

const parseLine = (line: any): string => {
  const valParsed = JSON.parse(line);
  // Sometimes it is just a string...
  if (typeof valParsed === 'string') {
    return valParsed;
  }
  return valParsed['text'];
};

const getSignsInChunk = (
  bot: Bot,
  chunkPos: Vec3,
  chunk: any
): ScanRegion | null => {
  const signs: Sign[] = [];

  for (const [posStr, blockEntityData] of Object.entries<any>(
    chunk.column.blockEntities
  )) {
    const frontTextMessages =
      blockEntityData?.['value']?.['front_text']?.['value']?.['messages']?.[
        'value'
      ]?.['value'];
    if (frontTextMessages) {
      const posElements = posStr.split(',').map((val: string) => parseInt(val));
      const vec = {
        x: posElements[0] + chunkPos.x,
        y: posElements[1] + chunkPos.y,
        z: posElements[2] + chunkPos.z
      };
      const lines = [
        parseLine(frontTextMessages[0]),
        parseLine(frontTextMessages[1]),
        parseLine(frontTextMessages[2]),
        parseLine(frontTextMessages[3])
      ];
      signs.push({
        location: { vec3: vec, dim: stringToDim(bot.game.dimension) },
        lines
      });
    }
  }

  return {
    signs,
    bounds: [
      { x: chunkPos.x, z: chunkPos.z },
      { x: chunkPos.x + 15, z: chunkPos.z + 15 }
    ],
    dimension: stringToDim(bot.game.dimension)
  };
};
