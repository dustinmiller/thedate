use actix_web::{HttpResponse, Responder};
use chrono::Utc;

use crate::timestamp::from_chrono;

pub async fn home() -> impl Responder {
    HttpResponse::Ok().json(from_chrono(Utc::now()))
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}
