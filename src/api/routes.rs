use actix_web::web::ServiceConfig;
use crate::api::health;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(health::health);
}