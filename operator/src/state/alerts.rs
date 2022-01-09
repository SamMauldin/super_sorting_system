use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone, Debug)]
pub enum AlertSource {
    Operator,
    Agent(Uuid),
}

#[derive(Serialize, Clone, Debug)]
pub struct Alert {
    pub source: AlertSource,
    pub description: String,
    pub timestamp: DateTime<Utc>,
}

pub struct AlertState {
    alerts: Vec<Alert>,
}

impl Default for AlertState {
    fn default() -> Self {
        AlertState {
            alerts: Default::default(),
        }
    }
}

impl AlertState {
    pub fn add_alert(&mut self, source: AlertSource, description: String) -> &Alert {
        self.alerts.push(Alert {
            source,
            description,
            timestamp: Utc::now(),
        });

        self.alerts.last().unwrap()
    }
}
