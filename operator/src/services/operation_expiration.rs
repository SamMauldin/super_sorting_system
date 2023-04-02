use crate::{config::Config, state::State};

use super::service::Service;

pub struct OperationExpirationService {}

impl Service for OperationExpirationService {
    fn new(_config: &Config) -> Self {
        Self {}
    }

    fn tick(&mut self, state: &mut State) {
        state.operations.purge_old_operations();
    }
}
