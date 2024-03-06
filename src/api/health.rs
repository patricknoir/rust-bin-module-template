use actix_web::{HttpResponse, Responder};

#[actix_web::get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}