use uuid::Uuid;

use super::service::Service;
use crate::{
    config::Config,
    state::inventories::{InventoryListingOptions, ShulkerUnpacking},
    state::operations::{OperationKind, OperationPriority, OperationStatus},
    state::State,
};

pub struct ShulkerLoaderService {
    outstanding_operation: Option<Uuid>,
}

impl Service for ShulkerLoaderService {
    fn new(_config: &Config) -> Self {
        ShulkerLoaderService {
            outstanding_operation: None,
        }
    }

    fn tick(&mut self, state: &mut State) {
        if let Some(op_id) = self.outstanding_operation {
            let op = state.operations.get(op_id);

            if let Some(op) = op {
                match op.status {
                    OperationStatus::Complete | OperationStatus::Aborted => {
                        for hold in op.holds() {
                            state.holds.remove(hold);
                        }

                        self.outstanding_operation = None
                    }
                    OperationStatus::Pending | OperationStatus::InProgress => return,
                }
            }
        }

        let inv_listing = state.inventories.get_listing(InventoryListingOptions {
            shulker_unpacking: ShulkerUnpacking::None,
        });

        for item in inv_listing.iter() {
            if item.shulker_data.is_some() {
                continue;
            }

            if item.count < 2 * item.stack_size * 27 {
                continue;
            }

            let mut full_stacks = vec![];

            for (loc, slot, inv_item) in state.inventories.iter_slots() {
                if full_stacks.len() == 27 {
                    break;
                }
                if let Some(inv_item) = inv_item {
                    if inv_item.stackable_hash != item.stackable_hash {
                        continue;
                    }

                    if inv_item.stack_size != inv_item.count {
                        continue;
                    }

                    if state.holds.existing_hold(loc, slot as u32).is_some() {
                        continue;
                    }

                    full_stacks.push((loc, slot));
                }
            }

            if full_stacks.len() == 27 {
                let mut holds = vec![];

                for (loc, slot) in full_stacks.into_iter() {
                    holds.push(Some(state.holds.create(loc, slot as u32).unwrap().id));
                }

                let sign_config = state.sign_config.get_config();

                let shulker_station_location = sign_config.nodes.iter().find_map(|(_, node)| {
                    if node.shulker_station {
                        return Some(node.location);
                    }

                    None
                });

                let shulker_station_location = if let Some(loc) = shulker_station_location {
                    loc
                } else {
                    return;
                };

                let empty_shulker = state.inventories.iter_slots().find(|(loc, slot, item)| {
                    item.as_ref()
                        .and_then(|item| item.shulker_data.as_ref())
                        .map_or(false, |shulker_data| {
                            shulker_data.empty && shulker_data.name.is_none()
                        })
                        && state.holds.existing_hold(*loc, *slot as u32).is_none()
                });

                let empty_shulker_hold = if let Some((loc, slot, _)) = empty_shulker {
                    state.holds.create(loc, slot as u32).unwrap().id
                } else {
                    return;
                };

                let queued_op_id = state
                    .operations
                    .queue_operation(
                        OperationPriority::Background,
                        OperationKind::LoadShulker {
                            shulker_station_location,
                            shulker_hold: empty_shulker_hold,
                            source_holds: holds,
                        },
                    )
                    .id;

                self.outstanding_operation = Some(queued_op_id);

                return;
            }
        }
    }
}
