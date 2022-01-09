export type Agent = {
  id: string;
  last_seen: string;
};

export type Vec3 = {
  x: number;
  y: number;
  z: number;
};

export const vecEq = (a: Vec3, b: Vec3) =>
  a.x === b.x && a.y === b.y && a.z === b.z;

export type Vec2 = Omit<Vec3, "y">;

export type ScanInventoryOperationKind = {
  type: "ScanInventory";
  location: Vec3;
};

export type MoveItemsOperationKind = {
  type: "MoveItems";
  source_hold: string;
  destination_hold: string;
  count: number;
};

export type DropItemsOperationKind = {
  type: "DropItems";
  drop_from: Vec3;
  aim_towards: Vec3;
  source_holds: string[];
};

export type ImportInventoryOperationKind = {
  type: "ImportInventory";
  chest_location: Vec3;
  node_location: Vec3;
  destination_holds: string[];
};

export type OperationKind =
  | ScanInventoryOperationKind
  | MoveItemsOperationKind
  | DropItemsOperationKind
  | ImportInventoryOperationKind;

export type Operation = {
  id: string;
  priority: string;
  status: string;
  kind: OperationKind;
};

export type Hold = {
  id: string;
  location: Vec3;
  slot: number;
  valid_until: string;
};

export type ComplexInfo = {
  dimension: string;
  y_level: number;
  bounds: [Vec2, Vec2];
};

export type Item = {
  item_id: number;
  count: number;
  metadata: number;
  nbt: any;
  stack_size: number;
};
