export type Vec3 = {
  x: number;
  y: number;
  z: number;
};

export const vecEq = (a: Vec3, b: Vec3) =>
  a.x === b.x && a.y === b.y && a.z === b.z;

export type Vec2 = Omit<Vec3, "y">;

export type Hold = {
  id: string;
  location: Vec3;
  slot: number;
  valid_until: string;
};

export type Item = {
  item_id: number;
  count: number;
  metadata: number;
  nbt: any;
  stack_size: number;
  stackable_hash: string;
};
