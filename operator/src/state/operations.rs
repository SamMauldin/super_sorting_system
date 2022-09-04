use crate::types::{Location, Vec3};
use serde::{Deserialize, Serialize};
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
    },
    ScanSigns {
        location: Location,
        take_portal: Option<Vec3>,
    },
    MoveItems {
        source_hold: Uuid,
        destination_hold: Uuid,
        count: u32,
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
    operations: Vec<Operation>,
}

impl Default for OperationState {
    fn default() -> Self {
        OperationState {
            operations: Default::default(),
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
        self.operations.push(Operation {
            id: Uuid::new_v4(),
            priority,
            status: OperationStatus::Pending,
            kind,
        });

        self.operations.last().unwrap()
    }

    pub fn take_next_operation(&mut self) -> Option<&Operation> {
        self.operations.sort_by(|a, b| a.priority.cmp(&b.priority));

        self.operations
            .iter_mut()
            .find(|op| matches!(op.status, OperationStatus::Pending))
            .map(|op| {
                op.status = OperationStatus::InProgress;

                &*op
            })
    }

    pub fn set_operation_status(
        &mut self,
        operation_id: Uuid,
        status: OperationStatus,
    ) -> Result<&Operation, OperationError> {
        self.operations
            .iter_mut()
            .find(|op| op.id == operation_id)
            .ok_or_else(|| OperationError::NotFound)
            .map(|op| {
                op.status = status;

                &*op
            })
    }

    pub fn iter(&self, status: OperationStatus) -> impl Iterator<Item = &Operation> {
        self.operations.iter().filter(move |op| op.status == status)
    }

    pub fn get(&self, id: Uuid) -> Option<&Operation> {
        self.operations.iter().find(|op| op.id == id)
    }
}

impl Operation {
    pub fn holds(&self) -> Vec<Uuid> {
        match &self.kind {
            OperationKind::ScanInventory { .. } | OperationKind::ScanSigns { .. } => vec![],
            OperationKind::MoveItems {
                source_hold,
                destination_hold,
                ..
            } => vec![*source_hold, *destination_hold],
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
}
