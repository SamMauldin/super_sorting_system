import { Dimension, Item, Loc, Vec2, Vec3 } from "./types";

export type InventoryWithLoc = {
  slots: Array<Item | null>;
  loc: Loc;
};

export type InventoriesWithLoc = Array<InventoryWithLoc>;

export type PathfindingNode = {
  location: Loc;
  name: string;
  connections: string[];
  pickup?: Vec3;
  dropoff?: Vec3;
};

export type StorageComplex = {
  dimension: Dimension;
  y_level: number;
  bounds: [Vec2, Vec2];
  name: string;
};

export type CompiledSignConfig = {
  nodes: { [name: string]: PathfindingNode };
  complexes: { [name: string]: StorageComplex };
};

export type Hold = {
  id: string;
  location: Loc;
  slot: number;
  valid_until: string;
};

export type OperationPriority =
  | "SystemCritical"
  | "UserInteractive"
  | "Background"
  | "LowPriority";

export type OperationStatus = "Pending" | "InProgress" | "Complete" | "Aborted";

export type ScanInventoryOperationKind = {
  type: "ScanInventory";
  location: Loc;
};

export type ScanSignsOperationKind = {
  type: "ScanSigns";
  location: Loc;
};

export type MoveItemsOperationKind = {
  type: "MoveItems";
  source_hold: string;
  destination_hold: string;
  count: number;
};

export type DropItemsOperationKind = {
  type: "DropItems";
  drop_from: Loc;
  aim_towards: Vec3;
  source_holds: string[];
};

export type ImportInventoryOperationKind = {
  type: "ImportInventory";
  chest_location: Vec3;
  node_location: Loc;
  destination_holds: string[];
};

export type OperationKind =
  | ScanInventoryOperationKind
  | ScanSignsOperationKind
  | MoveItemsOperationKind
  | DropItemsOperationKind
  | ImportInventoryOperationKind;

export type Operation = {
  id: string;
  priority: OperationPriority;
  status: OperationStatus;
  kind: OperationKind;
};
