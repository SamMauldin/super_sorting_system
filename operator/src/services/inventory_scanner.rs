use std::cmp::{max, min};

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::{
    config::Config,
    state::{
        operations::{OperationKind, OperationPriority, OperationStatus},
        sign_config::StorageComplex,
        State,
    },
    types::{Location, Vec3},
};

use super::service::Service;

struct TrackedInventory {
    location: Location,
    open_from: Vec3,
    current_scan_operation_id: Option<Uuid>,
}

pub struct InventoryScannerService {
    tracked_inventories: Vec<TrackedInventory>,
}

impl Service for InventoryScannerService {
    fn new(_config: &Config) -> Self {
        InventoryScannerService {
            tracked_inventories: Default::default(),
        }
    }

    fn tick(&mut self, state: &mut State) {
        let sign_config = state.sign_config.get_config();

        for (_name, complex) in sign_config.complexes.iter() {
            match complex {
                StorageComplex::FlatFloor {
                    dimension,
                    name: _name,
                    y_level,
                    bounds,
                } => {
                    let x1 = bounds.0.x;
                    let x2 = bounds.1.x;
                    let z1 = bounds.0.z;
                    let z2 = bounds.1.z;

                    for x in min(x1, x2)..=max(x1, x2) {
                        for z in min(z1, z2)..=max(z1, z2) {
                            if self.tracked_inventories.iter().any(|inv| {
                                inv.location
                                    == Location {
                                        vec3: Vec3 { x, y: *y_level, z },
                                        dim: *dimension,
                                    }
                            }) {
                                continue;
                            }

                            self.tracked_inventories.push(TrackedInventory {
                                location: Location {
                                    vec3: Vec3 { x, y: *y_level, z },
                                    dim: *dimension,
                                },
                                open_from: Vec3 {
                                    x,
                                    y: (*y_level + 1),
                                    z,
                                },
                                current_scan_operation_id: None,
                            })
                        }
                    }
                }
                StorageComplex::Tower {
                    dimension,
                    name: _name,
                    origin,
                    height,
                } => {
                    for y in (origin.y)..=(origin.y + (*height as i32) - 1) {
                        for x in (origin.x - 4)..=(origin.x + 4) {
                            for z in (origin.z - 4)..=(origin.z + 4) {
                                if x == origin.x && z == origin.z {
                                    continue;
                                }

                                if self.tracked_inventories.iter().any(|inv| {
                                    inv.location
                                        == Location {
                                            vec3: Vec3 { x, y, z },
                                            dim: *dimension,
                                        }
                                }) {
                                    continue;
                                }

                                self.tracked_inventories.push(TrackedInventory {
                                    location: Location {
                                        vec3: Vec3 { x, y, z },
                                        dim: *dimension,
                                    },
                                    open_from: Vec3 {
                                        x: origin.x,
                                        y,
                                        z: origin.z,
                                    },
                                    current_scan_operation_id: None,
                                })
                            }
                        }
                    }
                }
            }
        }

        for inventory in self.tracked_inventories.iter_mut() {
            if let Some(op_id) = inventory.current_scan_operation_id {
                let op = state.operations.get(op_id);

                match op {
                    Some(op) => match op.status {
                        OperationStatus::Complete | OperationStatus::Aborted => {
                            inventory.current_scan_operation_id = None
                        }
                        OperationStatus::InProgress | OperationStatus::Pending => continue,
                    },
                    None => inventory.current_scan_operation_id = None,
                }
            }

            let existing_inventory = state.inventories.inventory_contents_at(&inventory.location);

            let needs_rescan = match existing_inventory {
                Some(inventory) => inventory.scanned_at + Duration::hours(2) < Utc::now(),
                None => true,
            };

            if !needs_rescan {
                continue;
            }

            let priority = match existing_inventory {
                Some(_) => OperationPriority::Background,
                None => OperationPriority::SystemCritical,
            };

            let kind = OperationKind::ScanInventory {
                location: inventory.location,
                open_from: inventory.open_from,
            };

            let op = state.operations.queue_operation(priority, kind);

            inventory.current_scan_operation_id = Some(op.id);
        }
    }
}
