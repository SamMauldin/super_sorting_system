use std::{cmp::min, collections::HashMap};

use uuid::Uuid;

use super::service::Service;
use crate::{
    config::Config,
    state::operations::{OperationKind, OperationPriority, OperationStatus},
    state::State,
    types::{Item, Vec3},
};

pub struct DefraggerService {
    outstanding_operation: Option<Uuid>,
}

impl Service for DefraggerService {
    fn new(_config: &Config) -> Self {
        DefraggerService {
            outstanding_operation: None,
        }
    }

    fn tick(&mut self, state: &mut State) {
        if let Some(op_id) = self.outstanding_operation {
            let op = state.operations.get(op_id);

            if let Some(op) = op {
                match op.status {
                    OperationStatus::Complete | OperationStatus::Aborted => {
                        match op.kind {
                            OperationKind::MoveItems {
                                source_hold,
                                destination_hold,
                                count: _,
                            } => {
                                state.holds.remove(source_hold);
                                state.holds.remove(destination_hold);
                            }
                            _ => panic!("Operation kind changed!"),
                        }

                        self.outstanding_operation = None
                    }
                    OperationStatus::Pending | OperationStatus::InProgress => return,
                }
            }
        }

        let mut partial_items: HashMap<&str, (Vec3, usize, &Item)> = HashMap::new();

        for (loc, slot, item) in state.inventories.iter_slots() {
            if let Some(item) = item {
                if item.count < item.stack_size {
                    if state.holds.existing_hold(loc, slot as u32).is_some() {
                        continue;
                    }

                    if let Some((pair_loc, pair_slot, pair_item)) =
                        partial_items.get(item.stackable_hash.as_str())
                    {
                        let remaining_space = pair_item.stack_size - pair_item.count;
                        let items_to_move = min(item.count, remaining_space);

                        let hold_id = state.holds.create(loc, slot as u32).unwrap().id;
                        let pair_hold_id =
                            state.holds.create(*pair_loc, *pair_slot as u32).unwrap().id;

                        let queued_op_id = state
                            .operations
                            .queue_operation(
                                OperationPriority::Background,
                                OperationKind::MoveItems {
                                    source_hold: hold_id,
                                    destination_hold: pair_hold_id,
                                    count: items_to_move,
                                },
                            )
                            .id;

                        self.outstanding_operation = Some(queued_op_id);

                        return;
                    } else {
                        partial_items.insert(item.stackable_hash.as_str(), (loc, slot, item));
                    }
                }
            }
        }
    }
}
