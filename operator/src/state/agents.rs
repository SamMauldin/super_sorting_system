use std::collections::HashMap;

use actix_web::{dev, error::ErrorBadRequest, FromRequest, HttpRequest};
use chrono::{DateTime, Utc};
use futures_util::future::{err, ok, Ready};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

use crate::state::StateData;

#[derive(Serialize, Clone)]
pub struct Agent {
    pub id: Uuid,
    pub last_seen: DateTime<Utc>,
    pub current_operation: Option<Uuid>,
}

pub struct AgentState {
    agents: HashMap<Uuid, Agent>,
}

impl Default for AgentState {
    fn default() -> Self {
        AgentState {
            agents: Default::default(),
        }
    }
}

#[derive(Error, Serialize, Debug)]
pub enum AgentError {
    #[error("Could not find that agent")]
    NotFound,
}

impl AgentState {
    pub fn register(&mut self) -> &Agent {
        let id = Uuid::new_v4();

        self.agents.insert(
            id,
            Agent {
                id,
                last_seen: Utc::now(),
                current_operation: None,
            },
        );

        self.agents.get(&id).unwrap()
    }

    pub fn get_and_mark_seen(&mut self, id: Uuid) -> Result<&Agent, AgentError> {
        self.agents
            .get_mut(&id)
            .ok_or_else(|| AgentError::NotFound)
            .map(|agent| {
                agent.last_seen = Utc::now();

                &*agent
            })
    }

    pub fn remove(&mut self, id: Uuid) -> Result<Agent, AgentError> {
        self.agents.remove(&id).ok_or_else(|| AgentError::NotFound)
    }

    pub fn set_operation(&mut self, id: Uuid, op: Option<Uuid>) -> Result<(), AgentError> {
        self.agents
            .get_mut(&id)
            .ok_or_else(|| AgentError::NotFound)
            .map(|agent| {
                agent.current_operation = op;
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &Agent> {
        self.agents.iter().map(|(_id, agent)| agent)
    }
}

impl FromRequest for Agent {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let mut data = req.app_data::<StateData>().unwrap().lock().unwrap();

        let agent = req
            .headers()
            .get("X-Agent-Id")
            .ok_or_else(|| ErrorBadRequest("X-Agent-Id Header Not Provided"))
            .and_then(|agent_id| {
                agent_id
                    .to_str()
                    .map_err(|_err| ErrorBadRequest("Invalid Agent ID"))
            })
            .and_then(|agent_id| {
                Uuid::parse_str(agent_id).map_err(|_err| ErrorBadRequest("Invalid Agent ID"))
            })
            .and_then(|agent_id| {
                data.agents
                    .get_and_mark_seen(agent_id)
                    .map_err(|_err| ErrorBadRequest("Agent ID Not Found"))
            });

        match agent {
            Ok(agent) => ok(agent.clone()),
            Err(error) => err(error),
        }
    }
}
