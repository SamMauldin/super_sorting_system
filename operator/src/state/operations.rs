use crate::types::{Location, Vec3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum OperationPriority {
    SystemCritical = 0,
    UserInteractive = 1,
    Background = 2,
    LowPriority = 3,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperationStatus {
    Pending,
    InProgress,
    Complete,
    Aborted,
}

#[derive(Debug, Serialize, Clone)]
pub struct Operation {
    pub id: Uuid,
    pub priority: OperationPriority,
    pub status: OperationStatus,
    pub kind: OperationKind,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum OperationKind {
    ScanInventory {
        location: Location,
        open_from: Vec3,
    },
    ScanSigns {
        location: Location,
        take_portal: Option<Vec3>,
    },
    MoveItems {
        source_holds: Vec<Uuid>,
        destination_holds: Vec<Uuid>,
        counts: Vec<i32>,
    },
    DropItems {
        drop_from: Location,
        aim_towards: Vec3,
        source_holds: Vec<Uuid>,
    },
    ImportInventory {
        chest_location: Vec3,
        node_location: Location,
        destination_holds: Vec<Uuid>,
    },
    Craft {
        crafting_table_location: Vec3,
        node_location: Location,
        recipe_source_holds: Vec<Option<Uuid>>,
        destination_holds: Vec<Uuid>,
    },
    LoadShulker {
        shulker_station_location: Location,
        shulker_hold: Uuid,
        source_holds: Vec<Option<Uuid>>,
    },
    UnloadShulker {
        shulker_station_location: Location,
        shulker_hold: Uuid,
        destination_holds: Vec<Uuid>,
    },
}

pub struct OperationState {
    operations: HashMap<Uuid, Operation>,
    pending_operation_ids: Vec<(Uuid, OperationPriority)>,
}

impl Default for OperationState {
    fn default() -> Self {
        OperationState {
            operations: Default::default(),
            pending_operation_ids: Default::default(),
        }
    }
}

#[derive(Error, Serialize, Debug)]
pub enum OperationError {
    #[error("Could not find that operation")]
    NotFound,
}

impl OperationState {
    pub fn queue_operation(
        &mut self,
        priority: OperationPriority,
        kind: OperationKind,
    ) -> &Operation {
        let id = Uuid::new_v4();
        self.operations.insert(
            id,
            Operation {
                id,
                priority,
                status: OperationStatus::Pending,
                kind,
            },
        );

        self.operations.get(&id).unwrap()
    }

    pub fn take_next_operation(&mut self) -> Option<&Operation> {
        let shulker_stations_in_use = self
            .iter(OperationStatus::InProgress)
            .map(|op| op.shulker_station_location())
            .flatten()
            .collect::<Vec<Location>>();

        self.pending_operation_ids.sort_by(|a, b| a.1.cmp(&b.1));

        let next_op =
            self.pending_operation_ids
                .iter()
                .enumerate()
                .find(|(_idx, (op_id, _priority))| {
                    let op = self.operations.get(&op_id).unwrap();

                    op.shulker_station_location()
                        .as_ref()
                        .map(|station| !shulker_stations_in_use.contains(station))
                        .unwrap_or(true)
                });

        if let Some((idx, (op_id, _priority))) = next_op {
            let op_id = *op_id;
            self.operations.get_mut(&op_id).unwrap().status = OperationStatus::InProgress;
            self.pending_operation_ids.remove(idx);

            return self.operations.get(&op_id);
        } else {
            return None;
        }
    }

    pub fn set_operation_status(
        &mut self,
        operation_id: Uuid,
        status: OperationStatus,
    ) -> Result<&Operation, OperationError> {
        self.operations
            .get_mut(&operation_id)
            .ok_or_else(|| OperationError::NotFound)
            .map(|op| {
                op.status = status;

                &*op
            })
    }

    pub fn iter(&self, status: OperationStatus) -> impl Iterator<Item = &Operation> {
        self.operations
            .iter()
            .filter(move |(_id, op)| op.status == status)
            .map(|(_id, op)| op)
    }

    pub fn get(&self, id: Uuid) -> Option<&Operation> {
        self.operations.get(&id)
    }
}

impl Operation {
    pub fn holds(&self) -> Vec<Uuid> {
        match &self.kind {
            OperationKind::ScanInventory { .. } | OperationKind::ScanSigns { .. } => vec![],
            OperationKind::MoveItems {
                source_holds,
                destination_holds,
                ..
            } => source_holds
                .iter()
                .chain(destination_holds.iter())
                .map(|hold| *hold)
                .collect(),
            OperationKind::DropItems { source_holds, .. } => source_holds.clone(),
            OperationKind::ImportInventory {
                destination_holds, ..
            } => destination_holds.clone(),
            OperationKind::Craft {
                recipe_source_holds,
                destination_holds,
                ..
            } => {
                let mut holds = vec![];
                holds.extend(recipe_source_holds.iter().filter_map(|op_hold| *op_hold));
                holds.extend(destination_holds.iter());
                holds
            }
            OperationKind::LoadShulker {
                shulker_hold,
                source_holds,
                ..
            } => {
                let mut holds = vec![];
                holds.extend(source_holds.iter().filter_map(|op_hold| *op_hold));
                holds.push(*shulker_hold);
                holds
            }
            OperationKind::UnloadShulker {
                shulker_hold,
                destination_holds,
                ..
            } => {
                let mut holds = vec![];
                holds.extend(destination_holds.iter());
                holds.push(*shulker_hold);
                holds
            }
        }
    }

    pub fn shulker_station_location(&self) -> Option<Location> {
        match &self.kind {
            OperationKind::LoadShulker {
                shulker_station_location,
                ..
            } => Some(*shulker_station_location),
            OperationKind::UnloadShulker {
                shulker_station_location,
                ..
            } => Some(*shulker_station_location),
            _ => None,
        }
    }
}
