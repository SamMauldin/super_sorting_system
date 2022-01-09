export type ItemType = {
  id: number;
  displayName: string;
  name: string;
  stackSize: number;
  enchantCategories?: EnchantCategory[];
  maxDurability?: number;
  repairWith?: string[];
};

export enum EnchantCategory {
  Armor = "armor",
  ArmorChest = "armor_chest",
  ArmorFeet = "armor_feet",
  ArmorHead = "armor_head",
  Bow = "bow",
  Breakable = "breakable",
  Crossbow = "crossbow",
  Digger = "digger",
  FishingRod = "fishing_rod",
  Trident = "trident",
  Vanishable = "vanishable",
  Weapon = "weapon",
  Wearable = "wearable",
}

export type ItemTypeById = Map<number, ItemType>;

export interface EnchantmentType {
  id: number;
  name: string;
  displayName: string;
  maxLevel: number;
  minCost: EnchantmentCode;
  maxCost: EnchantmentCode;
  treasureOnly: boolean;
  curse: boolean;
  exclude: string[];
  category: string;
  weight: number;
  tradeable: boolean;
  discoverable: boolean;
}

export interface EnchantmentCode {
  a: number;
  b: number;
}

export type EnchantmentTypeByName = Map<string, EnchantmentType>;
