import { Decorator } from './types';
import { anvilNameDecorator } from './anvilName';
import { enchantmentsDecorator } from './enchantments';

export * from './types';

export const decorators: Decorator[] = [
  anvilNameDecorator,
  enchantmentsDecorator,
];
