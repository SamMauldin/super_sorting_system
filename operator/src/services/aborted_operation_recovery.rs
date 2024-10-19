use crate::{
    config::Config,
    state::{
        operations::{OperationKind, OperationPriority, OperationStatus},
        State,
    },
    types::Location,
};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::service::Service;

pub struct AbortedOperationRecoveryService {
    // IDs of already-processed aborted operations
    processed_operations: HashSet<Uuid>,
    // Key: operation id, value: holds to be released
    outstanding_operations: HashMap<Uuid, Vec<Uuid>>,
}

impl Service for AbortedOperationRecoveryService {
    fn get_name(&self) -> &'static str {
        "aborted_operation_recovery"
    }

    fn new(_config: &Config) -> Self {
        AbortedOperationRecoveryService {
            processed_operations: HashSet::new(),
            outstanding_operations: HashMap::new(),
        }
    }

    fn tick(&mut self, state: &mut State) {
        let new_processed_operations = state
            .operations
            .iter(OperationStatus::Aborted)
            .map(|op| op.id)
            .collect::<HashSet<Uuid>>();

        for op_id in new_processed_operations.iter() {
            if !self.processed_operations.contains(op_id) {
                let op = state.operations.get(*op_id).unwrap();

                let hold_ids = op
                    .holds()
                    .iter()
                    .flat_map(|hold_id| state.holds.takeover(*hold_id).map(|hold| hold.id))
                    .collect::<Vec<Uuid>>();

                // Location: (operation id, hold ids)
                let mut rescanned_locations: HashMap<Location, (Uuid, Vec<Uuid>)> = HashMap::new();
                for hold_id in hold_ids {
                    let hold = state.holds.get(hold_id).unwrap();
                    if let Some(existing_entry) = rescanned_locations.get_mut(&hold.location) {
                        existing_entry.1.push(hold_id);
                    } else {
                        let rescan_op = state.operations.queue_operation(
                            OperationPriority::SystemCritical,
                            OperationKind::ScanInventory {
                                location: hold.location,
                                open_from: hold.open_from,
                            },
                        );

                        rescanned_locations.insert(hold.location, (rescan_op.id, vec![hold.id]));
                    }
                }

                for (_, (op_id, hold_ids)) in rescanned_locations {
                    self.outstanding_operations.insert(op_id, hold_ids);
                }
            }
        }

        self.processed_operations = new_processed_operations;

        let mut finished_operations = Vec::new();
        for (op_id, hold_ids) in self.outstanding_operations.iter() {
            let op = state.operations.get(*op_id);

            if let Some(op) = op {
                match op.status {
                    OperationStatus::Complete | OperationStatus::Aborted => {
                        for hold in hold_ids {
                            state.holds.remove(*hold);
                        }

                        finished_operations.push(*op_id);
                    }
                    OperationStatus::Pending | OperationStatus::InProgress => return,
                }
            } else {
                for hold in hold_ids {
                    state.holds.remove(*hold);
                }

                finished_operations.push(*op_id);
            }
        }

        for op_id in finished_operations {
            self.outstanding_operations.remove(&op_id);
        }
    }
}
