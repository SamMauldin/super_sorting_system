use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    state::{
        holds::{Hold, HoldError},
        operations::{Operation, OperationKind, OperationPriority},
        StateData,
    },
    types::{Item, Location},
};

#[derive(Serialize)]
struct InventoryWithLoc {
    pub slots: Vec<Option<Item>>,
    pub loc: Location,
}

#[get("/inventory_contents")]
async fn inventory_contents(state: StateData) -> impl Responder {
    let state = state.lock().unwrap();

    let contents: Vec<InventoryWithLoc> = state
        .inventories
        .iter_inventories()
        .map(|(loc, inv)| InventoryWithLoc {
            slots: inv.slots.clone(),
            loc: *loc,
        })
        .collect();

    HttpResponse::Ok().json(contents)
}

#[get("/sign_config")]
async fn sign_config(state: StateData) -> impl Responder {
    let state = state.lock().unwrap();
    let sign_config = state.sign_config.get_config();

    HttpResponse::Ok().json(sign_config.as_ref())
}

#[derive(Serialize)]
struct HoldList<'a> {
    holds: &'a Vec<&'a Hold>,
}

#[get("/holds")]
async fn holds_index(state: StateData) -> impl Responder {
    let state = state.lock().unwrap();
    let holds = state.holds.iter().collect::<Vec<&Hold>>();

    HttpResponse::Ok().json(HoldList { holds: &holds })
}

#[derive(Deserialize)]
struct CreateHoldRequest {
    location: Location,
    slot: u32,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum CreateHoldResponse {
    HoldCreated { hold: Hold },
    Error(HoldError),
}

#[post("/holds")]
async fn holds_create(state: StateData, hold_req: web::Json<CreateHoldRequest>) -> impl Responder {
    let mut state = state.lock().unwrap();

    let hold = state.holds.create(hold_req.location, hold_req.slot);

    match hold {
        Ok(hold) => HttpResponse::Ok().json(CreateHoldResponse::HoldCreated { hold: hold.clone() }),
        Err(err) => HttpResponse::InternalServerError().json(CreateHoldResponse::Error(err)),
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum RemoveHoldResponse {
    HoldRemoved { hold: Hold },
    HoldNotFound,
}

#[delete("/holds/{hold_id}")]
async fn remove_hold(state: StateData, hold_id: web::Path<Uuid>) -> impl Responder {
    let mut state = state.lock().unwrap();

    let removed_hold = state.holds.remove(*hold_id);

    match removed_hold {
        Some(hold) => HttpResponse::Ok().json(RemoveHoldResponse::HoldRemoved { hold }),
        None => HttpResponse::NotFound().json(RemoveHoldResponse::HoldNotFound),
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum RenewHoldResponse {
    HoldRenewed { hold: Hold },
    HoldNotFound,
}

#[post("/holds/{hold_id}/renew")]
async fn renew_hold(state: StateData, hold_id: web::Path<Uuid>) -> impl Responder {
    let mut state = state.lock().unwrap();

    let renewed_hold = state.holds.renew(*hold_id);

    match renewed_hold {
        Some(hold) => {
            HttpResponse::Ok().json(RenewHoldResponse::HoldRenewed { hold: hold.clone() })
        }
        None => HttpResponse::NotFound().json(RenewHoldResponse::HoldNotFound),
    }
}

#[derive(Deserialize)]
struct CreateOperationRequest {
    priority: OperationPriority,
    kind: OperationKind,
}

#[derive(Serialize)]
struct CreateOperationResponse<'a> {
    operation: &'a Operation,
}

#[post("/operations")]
async fn create_operation(
    state: StateData,
    op_req: web::Json<CreateOperationRequest>,
) -> impl Responder {
    let mut state = state.lock().unwrap();

    let op = state
        .operations
        .queue_operation(op_req.priority, op_req.kind.clone());

    HttpResponse::Ok().json(CreateOperationResponse { operation: op })
}

#[derive(Serialize)]
struct GetOperationResponse<'a> {
    operation: &'a Operation,
}

#[get("/operations/{operation_id}")]
async fn get_operation(state: StateData, op_id: web::Path<Uuid>) -> impl Responder {
    let state = state.lock().unwrap();

    let op = state.operations.get(*op_id);

    match op {
        Some(op) => HttpResponse::Ok().json(GetOperationResponse { operation: op }),
        None => HttpResponse::NotFound().body(""),
    }
}

pub fn configure(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/automation")
            .service(inventory_contents)
            .service(sign_config)
            .service(holds_index)
            .service(holds_create)
            .service(remove_hold)
            .service(renew_hold)
            .service(create_operation)
            .service(get_operation),
    );
}
