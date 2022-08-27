import { Decorator } from './types';

export const enchantmentsDecorator: Decorator = (
  item,
  mcData,
): string | null => {
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

  if (enchantments.length === 0) return null;

  return enchantments.join(", ");
};
