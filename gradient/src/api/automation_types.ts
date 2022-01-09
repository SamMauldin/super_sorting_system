import { Item, Vec3 } from "./types";

export type InventoryWithLoc = {
  slots: Array<Item | null>;
  loc: Vec3;
};

export type InventoriesWithLoc = Array<InventoryWithLoc>;

export type PathfindingPortal = { location: Vec3; connects_to?: string };
export type PathfindingNode = {
  pretty_name?: string;
  dimension: string;
  location: Vec3;
  connections: string[];
  portal?: PathfindingPortal;
  chest?: Vec3;
};

export type Hold = {
  id: string;
  location: Vec3;
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
  priority: OperationPriority;
  status: OperationStatus;
  kind: OperationKind;
};
