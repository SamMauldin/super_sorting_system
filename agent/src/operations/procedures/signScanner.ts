import { Bot } from "mineflayer";
import { Agent, stringToDim, Vec3 } from "../../types";
import vec3 from "vec3";
import { ScanRegion, sendSignScanData, Sign } from "../../controllerApi";

export const sendVisibleSignData = async (bot: Bot, agent: Agent) => {
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
      z: parseInt(chunk.chunkZ) * 16,
    };

    const scanRegion = getSignsInChunk(bot, chunkPos);
    if (scanRegion) scannedRegions.push(scanRegion);

    await yieldTick();
  }

  return scannedRegions;
};

const getSignsInChunk = (bot: Bot, chunkPos: Vec3): ScanRegion | null => {
  const data = require("minecraft-data")(bot._client.version);

  const signBlockIds: number[] = Object.values(data.blocksByName)
    .filter((b: any) => b.name.includes("_sign"))
    .map((b: any) => b.id);

  const chunk = bot.world.getColumnAt(chunkPos);

  if (!chunk) return null;

  const signs: Sign[] = [];

  for (let x = 0; x < 16; x++) {
    for (let y = 0; y < 256; y++) {
      for (let z = 0; z < 16; z++) {
        const type = chunk.getBlockType({ x, y, z });

        if (signBlockIds.includes(type)) {
          const signBlock = bot.blockAt(
            vec3({ x: chunkPos.x + x, y, z: chunkPos.z + z })
          );
          if (!signBlock || !signBlock.signText) continue;

          signs.push({
            lines: signBlock.signText?.split("\n"),
            location: {
              vec3: { x: chunkPos.x + x, y, z: chunkPos.z + z },
              dim: stringToDim(bot.game.dimension),
            },
          });
        }
      }
    }
  }

  return {
    signs,
    bounds: [
      { x: chunkPos.x, z: chunkPos.z },
      { x: chunkPos.x + 15, z: chunkPos.z + 15 },
    ],
    dimension: stringToDim(bot.game.dimension),
  };
};
