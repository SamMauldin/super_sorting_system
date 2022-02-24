import assert from "assert";
import {
  createHold,
  getInventoryContents,
  removeHold,
  renewHold,
} from "../api/automation";
import { Item, Loc, locEq } from "../api/types";
import { searchFor, stackMatches } from ".";

const verifySlot = async (
  loc: Loc,
  slot: number,
  contents: Item | null,
  ignoreCount?: boolean
): Promise<boolean> => {
  const { data: inventories } = await getInventoryContents();

  const invAtLoc = inventories.find(({ loc: invLoc }) => locEq(loc, invLoc));
  if (!invAtLoc) return false;
  assert(invAtLoc, "Inventory not found at given location!");

  const item = invAtLoc.slots[slot];

  return stackMatches(item, contents, ignoreCount);
};

export const acquireHoldVerified = async (
  loc: Loc,
  slot: number,
  contents: Item | null,
  ignoreCount?: boolean
): Promise<string> => {
  const { data: hold } = await createHold(loc, slot);

  if (hold.type === "Error") throw new Error("Could not acquire hold");

  const verified = await verifySlot(loc, slot, contents, ignoreCount);

  if (verified) return hold.hold.id;

  await removeHold(hold.hold.id);

  throw new Error("Slot changed while acquiring hold!");
};

export const acquireFreeSpaces = async (count: number): Promise<string[]> => {
  const located: string[] = [];

  while (located.length < count) {
    const emptySpaces = await searchFor(null);

    if (emptySpaces.length === 0)
      throw new Error("Unable to reserve a free slot!");

    for (const emptySpace of emptySpaces) {
      if (located.length === count) break;

      const { loc, slot } = emptySpace;

      const hold = await acquireHoldVerified(loc, slot, null).catch(() => null);

      if (hold) located.push(hold);
    }
  }

  return located;
};

export const renewHolds = async (holds: string[]): Promise<void> => {
  for (const hold_id of holds) {
    const { data } = await renewHold(hold_id);
    assert(data.type === "HoldRenewed", "Unable to renew hold");
  }
};

export const releaseHolds = async (holds: string[]): Promise<void> => {
  for (const hold_id of holds) {
    await removeHold(hold_id);
  }
};
