import { Decorator } from './types';

export const potionDecorator: Decorator = (item): string | null => {
  const nbtPotionValue: string | undefined = item.nbt?.value?.Potion?.value;
  if (!nbtPotionValue) return null;

  const value = nbtPotionValue.split(':')[1];

  return value
    .split('_')
    .map((word) => word[0].toUpperCase() + word.substring(1))
    .join(' ');
};
