use crate::{
    config::Config,
    state::{operations::OperationStatus, State},
};
use chrono::Utc;
use uuid::Uuid;

use super::service::Service;

pub struct HoldExpirationService {}

impl Service for HoldExpirationService {
    fn new(_config: &Config) -> Self {
        HoldExpirationService {}
    }

    fn tick(&mut self, state: &mut State) {
        let holds_to_renew = state
            .operations
            .iter(OperationStatus::InProgress)
            .chain(state.operations.iter(OperationStatus::Pending))
            .flat_map(|op| op.holds())
            .collect::<Vec<Uuid>>();

        for hold_id in holds_to_renew {
            state.holds.renew(hold_id);
        }

        let holds_to_remove = state
            .holds
            .iter()
            .filter(|hold| hold.valid_until < Utc::now())
            .map(|hold| hold.id)
            .collect::<Vec<Uuid>>();

        for hold_id in holds_to_remove {
            state.holds.remove(hold_id);
        }
    }
}
