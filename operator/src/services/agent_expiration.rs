use chrono::{Duration, Utc};

use crate::{
    config::Config,
    state::State,
    state::{alerts::AlertSource, operations::OperationStatus},
};

use super::service::Service;

pub struct AgentExpirationService {}

impl Service for AgentExpirationService {
    fn get_name(&self) -> &'static str {
        "agent_expiration"
    }

    fn new(_config: &Config) -> Self {
        AgentExpirationService {}
    }

    fn tick(&mut self, state: &mut State) {
        let agents_to_remove: Vec<_> = state
            .agents
            .iter()
            .filter(|agent| agent.last_seen + Duration::minutes(1) < Utc::now())
            .map(|agent| agent.id)
            .collect();

        for agent_id in agents_to_remove {
            let agent = state.agents.remove(agent_id).unwrap();

            if let Some(op_id) = agent.current_operation {
                let operation = state
                    .operations
                    .set_operation_status(op_id, OperationStatus::Aborted)
                    .unwrap();

                state.alerts.add_alert(
                    AlertSource::Operator,
                    format!(
                        "Agent {} timed out while outstanding operation in progress. {:?}",
                        agent_id, operation
                    ),
                );
            }
        }
    }
}
