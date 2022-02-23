use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};

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

pub fn configure(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/data")
            .service(items)
            .service(enchantments)
            .service(recipes),
    );
}
