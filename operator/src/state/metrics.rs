use std::time::Duration;

pub struct MetricsState {
    pub services_tick_time: Option<Duration>,
}

impl Default for MetricsState {
    fn default() -> Self {
        Self {
            services_tick_time: None,
        }
    }
}
