import { Item } from '../api/types';
import { McData } from '../common';
import { decorators } from './decorators';

export type ExtendedItem = Item & {
  prettyPrinted: string;
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
  return items.map((item) => {
    const itemType = mcData.items.get(item!.item_id)!;

    const decoratorOutputs = [];

    for (const decorator of decorators) {
      const res = decorator(item, mcData);
      if (res) decoratorOutputs.push(res);
    }

    let prettyPrinted = itemType.displayName;

    if (decoratorOutputs.length > 0) {
      const decoratorText = decoratorOutputs.join(', ');

      prettyPrinted += ` (${decoratorText})`;
    }

    return {
      prettyPrinted,
      ...item,
    };
  });
};
