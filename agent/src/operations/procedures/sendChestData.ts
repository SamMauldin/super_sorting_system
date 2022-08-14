import { Chest } from 'mineflayer';
import { Window } from 'prismarine-windows';
import { inventoryScanned } from '../../controllerApi';
import { Agent, Item, Location } from '../../types';

export const sendChestData = async (
  chest: Chest & Window,
  location: Location,
  agent: Agent
) => {
  const items: Array<Item | null> = chest.slots
    .slice(0, chest.inventoryStart)
    .map((slot) => {
      if (!slot) return null;

      return {
        item_id: slot.type,
        count: slot.count,
        metadata: slot.metadata,
        nbt: slot.nbt,
        stack_size: slot.stackSize
      };
    });

  await inventoryScanned(items, location, agent);
};
