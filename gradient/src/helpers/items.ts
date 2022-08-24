import { InventoriesWithLoc } from '../api/automation_types';
import { Item } from '../api/types';
import { McData } from '../common';

export type ExtendedItem = Item & { prettyPrinted: string };

const anvilName = (item: Item): string | null => {
  const nbtDisplayNameJson = item.nbt?.value?.display?.value?.Name?.value;

  try {
    return JSON.parse(nbtDisplayNameJson).text;
  } catch (_) {
    return null;
  }
};

const enchantmentData = (mcData: McData, item: Item): string | null => {
  const nbtEnchantments =
    item.nbt?.value?.Enchantments || item.nbt?.value?.StoredEnchantments;
  if (!nbtEnchantments) return null;

  const enchantments = nbtEnchantments.value.value.map((enchant: any) => {
    const name = enchant.id.value.split(':')[1];
    const level = enchant.lvl.value;

    const enchantmentType = mcData.enchantments.get(name);

    const displayName = enchantmentType?.displayName || name;

    return `${displayName} ${level}`;
  });

  return enchantments.join(', ');
};

export const prettyPrint = (mcData: McData, item: Item): string => {
  const itemType = mcData.items.get(item!.item_id)!;

  const enchants = enchantmentData(mcData, item);
  const anvilDisplayName = anvilName(item);

  let text = itemType.displayName;

  if (anvilDisplayName) {
    text += ` "${anvilDisplayName}"`;
  }

  if (enchants) {
    text += ` (${enchants})`;
  }

  return text;
};

export const stackMatches = (
  stackA: Item | null,
  stackB: Item | null,
  ignoreCount?: boolean,
): boolean => {
  if (stackA === null || stackB === null) return stackA === stackB;

  if (stackA.stackable_hash !== stackB.stackable_hash) return false;

  if (ignoreCount) return true;

  return stackA.count === stackB.count;
};

export const itemListFromInventories = (
  mcData: McData,
  items: Item[],
): ExtendedItem[] => {
  return items.map((item) => ({
    prettyPrinted: prettyPrint(mcData, item),
    ...item,
  }));
};
