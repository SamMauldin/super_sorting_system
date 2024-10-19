use super::service::Service;
use crate::{
    config::Config,
    state::inventories::{InventoryListingOptions, ShulkerUnpacking},
    state::operations::{OperationKind, OperationPriority, OperationStatus},
    state::State,
};
use uuid::Uuid;

pub struct ShulkerUnloaderService {
    outstanding_operation: Option<Uuid>,
}

impl Service for ShulkerUnloaderService {
    fn get_name(&self) -> &'static str {
        "shulker_unloader"
    }

    fn new(_config: &Config) -> Self {
        ShulkerUnloaderService {
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

        'shulker: for (loc, slot, item, open_from) in state.inventories.iter_slots() {
            if let Some(item) = item {
                if let Some(shulker_data) = &item.shulker_data {
                    if shulker_data.name.is_some() {
                        continue;
                    }

                    if state.holds.existing_hold(loc, slot as u32).is_some() {
                        continue;
                    }

                    if shulker_data.contained_items.len() == 0 {
                        continue;
                    }

                    let first_item_hash =
                        &shulker_data.contained_items.first().unwrap().stackable_hash;
                    let mut contains_one_type = true;

                    for item in shulker_data.contained_items.iter() {
                        if &item.stackable_hash != first_item_hash {
                            contains_one_type = false;
                            continue;
                        }
                    }

                    let is_full = shulker_data.contained_items.len() == 27
                        && shulker_data
                            .contained_items
                            .iter()
                            .find(|i| i.stack_size != i.count)
                            .is_none();

                    if contains_one_type && is_full {
                        for item in inv_listing.iter() {
                            if &item.stackable_hash == first_item_hash {
                                if item.count >= item.stack_size * 27 {
                                    continue 'shulker;
                                }
                            }
                        }
                    }

                    let shulker_hold_id =
                        state.holds.create(loc, slot as u32, open_from).unwrap().id;
                    let mut destination_hold_ids = vec![];

                    for _ in 0..27 {
                        for (loc, slot, item, open_from) in state.inventories.iter_slots() {
                            if item.is_some()
                                || state.holds.existing_hold(loc, slot as u32).is_some()
                            {
                                continue;
                            }

                            let hold = state.holds.create(loc, slot as u32, open_from).unwrap();

                            destination_hold_ids.push(hold.id);

                            break;
                        }
                    }

                    if destination_hold_ids.len() < 27 {
                        for hold in destination_hold_ids.iter() {
                            state.holds.remove(*hold);
                        }

                        return;
                    }

                    let sign_config = state.sign_config.get_config();

                    let shulker_station_location =
                        sign_config.nodes.iter().find_map(|(_, node)| {
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

                    let queued_op_id = state
                        .operations
                        .queue_operation(
                            OperationPriority::Background,
                            OperationKind::UnloadShulker {
                                shulker_station_location,
                                shulker_hold: shulker_hold_id,
                                destination_holds: destination_hold_ids,
                            },
                        )
                        .id;

                    self.outstanding_operation = Some(queued_op_id);

                    break;
                }
            }
        }
    }
}
