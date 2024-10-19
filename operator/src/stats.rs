use serde::Serialize;
use std::collections::HashMap;

use crate::{state::operations::OperationStatus, state::State};

#[derive(Debug, Serialize)]
pub struct Stats {
    pub inventories_in_mem: usize,
    pub total_slots: usize,
    pub free_slots: usize,

    pub current_holds: usize,

    pub operations_pending: usize,
    pub operations_in_progress: usize,
    pub operations_complete: usize,
    pub operations_aborted: usize,

    pub agents_connected: usize,

    pub services_tick_times_micros: HashMap<&'static str, u64>,
}

pub fn calculate_stats(state: &State) -> Stats {
    let inventories_in_mem = state.inventories.iter_inventories().count();

    let total_slots = state.inventories.iter_slots().count();
    let free_slots = state
        .inventories
        .iter_slots()
        .filter(|(_, _, slot, _)| slot.is_none())
        .count();

    let current_holds = state.holds.iter().count();

    let operations_pending = state.operations.iter(OperationStatus::Pending).count();
    let operations_in_progress = state.operations.iter(OperationStatus::InProgress).count();
    let operations_complete = state.operations.iter(OperationStatus::Complete).count();
    let operations_aborted = state.operations.iter(OperationStatus::Aborted).count();

    let agents_connected = state.agents.iter().count();

    let services_tick_times_micros = state.metrics.services_tick_time.as_ref().map_or_else(
        || HashMap::new(),
        |dur_map| {
            let mut mapped_map = HashMap::new();

            for (name, dur) in dur_map.iter() {
                mapped_map.insert(*name, dur.as_micros() as u64);
            }

            mapped_map
        },
    );

    Stats {
        inventories_in_mem,
        total_slots,
        free_slots,

        current_holds,

        operations_pending,
        operations_in_progress,
        operations_complete,
        operations_aborted,

        agents_connected,

        services_tick_times_micros,
    }
}
