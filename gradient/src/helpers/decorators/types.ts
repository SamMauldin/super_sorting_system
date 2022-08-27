import { Item } from '../../api/types';
import { McData } from '../../common';

export type Decorator = (item: Item, mcData: McData) => string | null;
