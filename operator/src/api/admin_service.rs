use actix_web::{get, guard, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{config::Config, state::StateData, stats::calculate_stats};

#[get("/stats")]
async fn stats(state: StateData) -> impl Responder {
    let state = state.lock().unwrap();

    let stats = calculate_stats(&state);

    HttpResponse::Ok().json(stats)
}

pub fn configure(app: &mut web::ServiceConfig, config: &Config) {
    let api_keys = config.auth.admin_api_keys.clone();

    app.service(
        web::scope("/admin")
            .guard(guard::fn_guard(move |req| {
                req.headers()
                    .get("X-Api-Key")
                    .and_then(|header| header.to_str().ok())
                    .and_then(|header| Uuid::parse_str(header).ok())
                    .map(|header| api_keys.contains(&header))
                    .unwrap_or(false)
            }))
            .service(stats),
    );
}
