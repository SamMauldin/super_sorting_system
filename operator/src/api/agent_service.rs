use std::ops::DerefMut;

use actix_web::{get, guard, post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::{ComplexConfig, Config, ConfigData},
    pathfinding::PathfindingError,
    state::{
        agents::Agent,
        alerts::{Alert, AlertSource},
        holds::Hold,
        operations::{Operation, OperationError, OperationStatus},
        StateData,
    },
    types::{Inventory, UnhashedItem, Vec3},
};

#[derive(Serialize)]
struct RegisterAgentResponse {
    agent: Agent,
    complex: ComplexConfig,
}

#[post("/register")]
async fn register_agent(state: StateData, config: ConfigData) -> impl Responder {
    let mut state = state.lock().unwrap();

    let agent = state.agents.register().clone();

    HttpResponse::Ok().json(RegisterAgentResponse {
        agent,
        complex: config.complex.clone(),
    })
}

#[post("/heartbeat")]
async fn heartbeat(_agent: Agent) -> impl Responder {
    HttpResponse::Ok().body("success")
}

#[derive(Deserialize)]
pub struct AgentAlertRequest {
    description: String,
}

#[derive(Serialize)]
pub struct AgentAlertResponse {
    alert: Alert,
}

#[post("/alert")]
async fn alert(
    agent: Agent,
    state: StateData,
    alert_request: web::Json<AgentAlertRequest>,
) -> impl Responder {
    let mut state = state.lock().unwrap();

    let alert = state
        .alerts
        .add_alert(
            AlertSource::Agent(agent.id),
            alert_request.description.to_owned(),
        )
        .clone();

    HttpResponse::Ok().json(AgentAlertResponse { alert })
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum PollOperationResponse {
    OperationAvailable { operation: Operation },
    OperationUnavailable,
}

#[post("/poll_operation")]
async fn poll_operation(agent: Agent, state: StateData) -> impl Responder {
    let mut state = state.lock().unwrap();

    if agent.current_operation.is_some() {
        return HttpResponse::Conflict().body("Agent already is executing an operation");
    }

    let next_operation = state.operations.take_next_operation().map(|op| op.clone());

    HttpResponse::Ok().json(match next_operation {
        Some(op) => {
            state.agents.set_operation(agent.id, Some(op.id)).unwrap();

            PollOperationResponse::OperationAvailable {
                operation: op.clone(),
            }
        }
        None => PollOperationResponse::OperationUnavailable,
    })
}

#[derive(Serialize)]
struct HoldResponse {
    hold: Hold,
}

#[get("/hold/{hold_id}")]
async fn get_hold(_agent: Agent, state: StateData, hold_id: web::Path<Uuid>) -> impl Responder {
    let state = state.lock().unwrap();

    let hold = state.holds.get(hold_id.into_inner());

    match hold {
        Some(hold) => HttpResponse::Ok().json(HoldResponse { hold: hold.clone() }),
        None => HttpResponse::NotFound().body(""),
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum FreeHoldResponse {
    HoldAcquired { hold: Hold },
    HoldUnavailable,
}

#[post("/hold/free")]
async fn free_hold(_agent: Agent, state: StateData) -> impl Responder {
    let mut state_lock = state.lock().unwrap();
    let state = state_lock.deref_mut();

    for (loc, slot, item) in state.inventories.iter_slots() {
        if item.is_some() || state.holds.existing_hold(loc, slot as u32).is_some() {
            continue;
        }

        let hold = state.holds.create(loc, slot as u32).unwrap();

        return HttpResponse::Ok().json(FreeHoldResponse::HoldAcquired { hold: hold.clone() });
    }

    HttpResponse::NotFound().json(FreeHoldResponse::HoldUnavailable)
}

#[derive(Deserialize)]
struct OperationCompleteRequest {
    operation_id: Uuid,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum OperationCompleteResponse {
    OperationCompleted { operation: Operation },
    Error(OperationError),
}

#[post("/operation_complete")]
async fn operation_complete(
    agent: Agent,
    state: StateData,
    operation_data: web::Json<OperationCompleteRequest>,
) -> impl Responder {
    let mut state = state.lock().unwrap();

    if agent.current_operation != Some(operation_data.operation_id) {
        return HttpResponse::BadRequest()
            .body("Given operation does not match currently executing operation (if any)");
    }

    state.agents.set_operation(agent.id, None).unwrap();

    let res = state
        .operations
        .set_operation_status(operation_data.operation_id, OperationStatus::Complete);

    match res {
        Ok(op) => HttpResponse::Ok().json(OperationCompleteResponse::OperationCompleted {
            operation: op.clone(),
        }),
        Err(error) => HttpResponse::BadRequest().json(OperationCompleteResponse::Error(error)),
    }
}

#[derive(Deserialize)]
pub struct InventoryScannedRequest {
    location: Vec3,
    slots: Vec<Option<UnhashedItem>>,
}

#[post("/inventory_scanned")]
async fn inventory_scanned(
    _agent: Agent,
    state: StateData,
    inventory_data: web::Json<InventoryScannedRequest>,
) -> impl Responder {
    let mut state = state.lock().unwrap();

    state.inventories.set_inventory_at(
        inventory_data.location,
        Inventory {
            slots: inventory_data
                .into_inner()
                .slots
                .into_iter()
                .map(|slot| match slot {
                    None => None,
                    Some(unhashed_item) => Some(unhashed_item.into_item()),
                })
                .collect(),
            scanned_at: Utc::now(),
        },
    );

    HttpResponse::Ok()
}

#[derive(Deserialize, Debug)]
pub struct PathfindingRequest {
    start_vec: Vec3,
    start_dim: String,
    end_vec: Vec3,
    end_dim: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum PathfindingResponse {
    PathFound { path: Vec<Vec3> },
    Error(PathfindingError),
}

#[post("/pathfinding")]
async fn pathfinding(
    _agent: Agent,
    req: web::Json<PathfindingRequest>,
    config: web::Data<Config>,
) -> impl Responder {
    let path = crate::pathfinding::find_path(
        req.start_vec,
        &req.start_dim,
        req.end_vec,
        &req.end_dim,
        &config,
    );

    match path {
        Ok(path) => HttpResponse::Ok().json(PathfindingResponse::PathFound { path }),
        Err(err) => HttpResponse::InternalServerError().json(PathfindingResponse::Error(err)),
    }
}

pub fn configure(app: &mut web::ServiceConfig, config: &Config) {
    let api_keys = config.auth.agent_api_keys.clone();

    app.service(
        web::scope("/agent")
            .guard(guard::fn_guard(move |req| {
                req.headers()
                    .get("X-Api-Key")
                    .and_then(|header| header.to_str().ok())
                    .and_then(|header| Uuid::parse_str(header).ok())
                    .map(|header| api_keys.contains(&header))
                    .unwrap_or(false)
            }))
            .service(register_agent)
            .service(heartbeat)
            .service(alert)
            .service(poll_operation)
            .service(get_hold)
            .service(free_hold)
            .service(operation_complete)
            .service(inventory_scanned)
            .service(pathfinding),
    );
}
