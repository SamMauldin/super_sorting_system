use crate::{config::Config, state::State};

use super::service::Service;

pub struct AlertExpirationService {}

impl Service for AlertExpirationService {
    fn new(_config: &Config) -> Self {
        Self {}
    }

    fn tick(&mut self, state: &mut State) {
        state.alerts.purge_old_alerts();
    }
}
