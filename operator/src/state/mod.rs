use self::{
    agents::AgentState, alerts::AlertState, holds::HoldState, inventories::InventoryState,
    metrics::MetricsState, operations::OperationState, sign_config::SignConfigState,
};
use actix_web::web;
use std::sync::Mutex;

pub mod agents;
pub mod alerts;
pub mod holds;
pub mod inventories;
pub mod metrics;
pub mod operations;
pub mod sign_config;

pub struct State {
    pub inventories: InventoryState,
    pub operations: OperationState,
    pub agents: AgentState,
    pub alerts: AlertState,
    pub holds: HoldState,
    pub sign_config: SignConfigState,
    pub metrics: MetricsState,
}

impl Default for State {
    fn default() -> State {
        State {
            inventories: Default::default(),
            operations: Default::default(),
            agents: Default::default(),
            alerts: Default::default(),
            holds: Default::default(),
            sign_config: Default::default(),
            metrics: Default::default(),
        }
    }
}

pub type StateData = web::Data<Mutex<State>>;
