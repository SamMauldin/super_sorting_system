use std::ops::DerefMut;

use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    pathfinding::PathfindingError,
    state::{
        agents::Agent,
        alerts::{Alert, AlertSource},
        holds::Hold,
        operations::{Operation, OperationError, OperationStatus},
        sign_config::Sign,
        StateData,
    },
    types::{Dimension, Inventory, Location, UnhashedItem, Vec2, Vec3},
};

#[derive(Serialize)]
struct RegisterAgentResponse {
    agent: Agent,
}

#[post("/register")]
async fn register_agent(state: StateData) -> impl Responder {
    let mut state = state.lock().unwrap();

    let agent = state.agents.register().clone();

    HttpResponse::Ok().json(RegisterAgentResponse { agent })
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
    final_status: OperationStatus,
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

    if !(operation_data.final_status == OperationStatus::Complete
        || operation_data.final_status == OperationStatus::Aborted)
    {
        return HttpResponse::BadRequest().body("Invalid final operation status given");
    }

    state.agents.set_operation(agent.id, None).unwrap();

    let res = state
        .operations
        .set_operation_status(operation_data.operation_id, operation_data.final_status);

    match res {
        Ok(op) => {
            let op = op.clone();

            state.alerts.add_alert(
                AlertSource::Agent(agent.id),
                format!("Operation {} failed", op.id),
            );

            HttpResponse::Ok().json(OperationCompleteResponse::OperationCompleted { operation: op })
        }
        Err(error) => HttpResponse::BadRequest().json(OperationCompleteResponse::Error(error)),
    }
}

#[derive(Deserialize)]
pub struct InventoryScannedRequest {
    location: Location,
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
    start_loc: Location,
    end_loc: Location,
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
    state: StateData,
) -> impl Responder {
    let state = state.lock().unwrap();

    let path = crate::pathfinding::find_path(req.start_loc, req.end_loc, &state);

    match path {
        Ok(path) => HttpResponse::Ok().json(PathfindingResponse::PathFound { path }),
        Err(err) => HttpResponse::InternalServerError().json(PathfindingResponse::Error(err)),
    }
}

#[derive(Deserialize)]
pub struct ScanRegion {
    signs: Vec<Sign>,
    bounds: (Vec2, Vec2),
    dimension: Dimension,
}

#[derive(Deserialize)]
pub struct SignScanDataRequest {
    scan_regions: Vec<ScanRegion>,
}

#[post("/sign_scan_data")]
async fn sign_scan_data(
    _agent: Agent,
    state: StateData,
    req: web::Json<SignScanDataRequest>,
) -> impl Responder {
    let mut state = state.lock().unwrap();

    for scan_region in req.into_inner().scan_regions.into_iter() {
        state.sign_config.clear_area(
            scan_region.dimension,
            scan_region.bounds.0,
            scan_region.bounds.1,
        );

        for sign in scan_region.signs.into_iter() {
            state.sign_config.add_sign(sign);
        }
    }

    HttpResponse::Ok()
}

pub fn configure(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/agent")
            .service(register_agent)
            .service(heartbeat)
            .service(alert)
            .service(poll_operation)
            .service(get_hold)
            .service(free_hold)
            .service(operation_complete)
            .service(inventory_scanned)
            .service(pathfinding)
            .service(sign_scan_data),
    );
}
