use std::collections::HashMap;
use std::time::Duration;

pub struct MetricsState {
    pub services_tick_time: Option<HashMap<&'static str, Duration>>,
}

impl Default for MetricsState {
    fn default() -> Self {
        Self {
            services_tick_time: None,
        }
    }
}
