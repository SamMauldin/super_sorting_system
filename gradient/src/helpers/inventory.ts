import { getHolds, getInventoryContents } from "../api/automation";
import { Item, Vec3, vecEq } from "../api/types";
import { stackMatches } from ".";

export const searchFor = async (
  item: Item | null,
  ignoreCount?: boolean
): Promise<Array<{ loc: Vec3; slot: number; contents: Item | null }>> => {
  const { data: inventories } = await getInventoryContents();
  const { data: holds } = await getHolds();

  const res: Array<{ loc: Vec3; slot: number; contents: Item | null }> = [];

  for (const { loc, slots } of inventories) {
    for (let slotIdx = 0; slotIdx < slots.length; slotIdx++) {
      if (!stackMatches(slots[slotIdx], item, ignoreCount)) continue;
      if (
        holds.holds.find(
          (hold) => vecEq(loc, hold.location) && hold.slot === slotIdx
        )
      )
        continue;

      res.push({ loc, slot: slotIdx, contents: slots[slotIdx] });
    }
  }

  return res;
};
