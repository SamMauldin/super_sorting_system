use actix_web::{get, web, HttpResponse, Responder};

use crate::{state::StateData, stats::calculate_stats};

#[get("/stats")]
async fn stats(state: StateData) -> impl Responder {
    let state = state.lock().unwrap();

    let stats = calculate_stats(&state);

    HttpResponse::Ok().json(stats)
}

pub fn configure(app: &mut web::ServiceConfig) {
    app.service(web::scope("/admin").service(stats));
}
