use crate::{config::Config, state::State};

use super::service::Service;

pub struct AlertExpirationService {}

impl Service for AlertExpirationService {
    fn get_name(&self) -> &'static str {
        "alert_expiration"
    }

    fn new(_config: &Config) -> Self {
        Self {}
    }

    fn tick(&mut self, state: &mut State) {
        state.alerts.purge_old_alerts();
    }
}
