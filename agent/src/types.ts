export type Agent = {
  id: string;
  last_seen: string;
};

export type Vec3 = {
  x: number;
  y: number;
  z: number;
};

export type Dimension = 'Overworld' | 'TheNether' | 'TheEnd';

export type PfResultNode =
  | {
      Vec: Vec3;
    }
  | {
      Portal: {
        vec: Vec3;
        destination_dim: Dimension;
      };
    };

export const stringToDim = (dim: string): Dimension => {
  if (dim === 'overworld') {
    return 'Overworld';
  } else if (dim === 'the_nether') {
    return 'TheNether';
  } else if (dim === 'the_end') {
    return 'TheEnd';
  } else {
    throw new Error(`Unknown dimension ${dim}`);
  }
};

export const dimToString = (dim: Dimension): string => {
  if (dim === 'Overworld') {
    return 'overworld';
  } else if (dim === 'TheNether') {
    return 'the_nether';
  } else if (dim === 'TheEnd') {
    return 'the_end';
  } else {
    throw new Error(`Unknown dimension ${dim}`);
  }
};

export type Location = {
  vec3: Vec3;
  dim: Dimension;
};

export const vecEq = (a: Vec3, b: Vec3) =>
  a.x === b.x && a.y === b.y && a.z === b.z;

export const locEq = (a: Location, b: Location) =>
  vecEq(a.vec3, b.vec3) && a.dim === b.dim;

export type Vec2 = Omit<Vec3, 'y'>;

export type ScanInventoryOperationKind = {
  type: 'ScanInventory';
  location: Location;
  open_from: Vec3;
};

export type ScanSignsOperationKind = {
  type: 'ScanSigns';
  location: Location;
  take_portal?: Vec3;
};

export type MoveItemsOperationKind = {
  type: 'MoveItems';
  source_holds: [string];
  destination_holds: [string];
  counts: [number];
};

export type DropItemsOperationKind = {
  type: 'DropItems';
  drop_from: Location;
  aim_towards: Vec3;
  source_holds: string[];
};

export type ImportInventoryOperationKind = {
  type: 'ImportInventory';
  chest_location: Vec3;
  node_location: Location;
  destination_holds: string[];
};

export type LoadShulkerOperationKind = {
  type: 'LoadShulker';
  shulker_station_location: Location;
  shulker_hold: string;
  source_holds: (string | null)[];
};

export type UnloadShulkerOperationKind = {
  type: 'UnloadShulker';
  shulker_station_location: Location;
  shulker_hold: string;
  destination_holds: string[];
};

export type OperationKind =
  | ScanInventoryOperationKind
  | ScanSignsOperationKind
  | MoveItemsOperationKind
  | DropItemsOperationKind
  | ImportInventoryOperationKind
  | UnloadShulkerOperationKind
  | LoadShulkerOperationKind;

export type OperationStatus = 'Pending' | 'InProgress' | 'Complete' | 'Aborted';

export type Operation = {
  id: string;
  priority: string;
  status: OperationStatus;
  kind: OperationKind;
};

export type Hold = {
  id: string;
  location: Location;
  slot: number;
  valid_until: string;
  open_from: Vec3;
};

export type Item = {
  item_id: number;
  count: number;
  metadata: number;
  nbt: any;
  stack_size: number;
};
