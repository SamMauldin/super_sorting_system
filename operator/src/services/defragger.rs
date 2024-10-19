use hashbrown::HashMap;
use std::cmp::min;

use uuid::Uuid;

use super::service::Service;
use crate::{
    config::Config,
    state::operations::{OperationKind, OperationPriority, OperationStatus},
    state::State,
    types::{Item, Location, Vec3},
};

pub struct DefraggerService {
    outstanding_operation: Option<Uuid>,
}

impl Service for DefraggerService {
    fn get_name(&self) -> &'static str {
        "defragger"
    }

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
                        op.holds().iter().for_each(|hold| {
                            state.holds.remove(*hold);
                        });

                        self.outstanding_operation = None
                    }
                    OperationStatus::Pending | OperationStatus::InProgress => return,
                }
            }
        }

        let mut partial_items: HashMap<u64, (Location, usize, &Item, Vec3)> = HashMap::new();

        for (loc, slot, item, open_from) in state.inventories.iter_slots() {
            if let Some(item) = item {
                if item.count < item.stack_size {
                    if state.holds.existing_hold(loc, slot as u32).is_some() {
                        continue;
                    }

                    if let Some((pair_loc, pair_slot, pair_item, pair_open_from)) =
                        partial_items.get(&item.stackable_hash)
                    {
                        let remaining_space = pair_item.stack_size - pair_item.count;
                        let items_to_move = min(item.count, remaining_space);

                        let hold_id = state.holds.create(loc, slot as u32, open_from).unwrap().id;
                        let pair_hold_id = state
                            .holds
                            .create(*pair_loc, *pair_slot as u32, *pair_open_from)
                            .unwrap()
                            .id;

                        let queued_op_id = state
                            .operations
                            .queue_operation(
                                OperationPriority::Background,
                                OperationKind::MoveItems {
                                    source_holds: vec![hold_id],
                                    destination_holds: vec![pair_hold_id],
                                    counts: vec![items_to_move as i32],
                                },
                            )
                            .id;

                        self.outstanding_operation = Some(queued_op_id);

                        return;
                    } else {
                        partial_items.insert(item.stackable_hash, (loc, slot, item, open_from));
                    }
                }
            }
        }
    }
}
