import { Decorator } from './types';
import { anvilNameDecorator } from './anvilName';
import { enchantmentsDecorator } from './enchantments';
import { shulkerDecorator } from './shulker';
import { beeNestDecorator } from './beeNest';
import { potionDecorator } from './potion';

export * from './types';

export const decorators: Decorator[] = [
  anvilNameDecorator,
  enchantmentsDecorator,
  shulkerDecorator,
  beeNestDecorator,
  potionDecorator,
];
