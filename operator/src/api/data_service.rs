use actix_web::{get, guard, http::header::ContentType, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::config::Config;

#[get("/items")]
async fn items() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(include_str!("../../assets/minecraft-data/items.json"))
}

#[get("/enchantments")]
async fn enchantments() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(include_str!(
            "../../assets/minecraft-data/enchantments.json"
        ))
}

#[get("/recipes")]
async fn recipes() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(include_str!("../../assets/minecraft-data/recipes.json"))
}

pub fn configure(app: &mut web::ServiceConfig, config: &Config) {
    let api_keys = config.auth.data_api_keys.clone();

    app.service(
        web::scope("/data")
            .guard(guard::fn_guard(move |req| {
                req.headers()
                    .get("X-Api-Key")
                    .and_then(|header| header.to_str().ok())
                    .and_then(|header| Uuid::parse_str(header).ok())
                    .map(|header| api_keys.contains(&header))
                    .unwrap_or(false)
            }))
            .service(items)
            .service(enchantments)
            .service(recipes),
    );
}
