import { Decorator } from './types';

export const anvilNameDecorator: Decorator = (item) => {
  const nbtDisplayNameJson = item.nbt?.value?.display?.value?.Name?.value;

  try {
    const text = JSON.parse(nbtDisplayNameJson).text;

    if (text && text.length > 0) return `"${text}"`;
  } catch (_) {}

  return null;
};
