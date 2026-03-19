use axum::{extract::Json, http::StatusCode};
use argus_core::models::Event;

pub async fn collect_event(Json(event): Json<Event>) -> StatusCode {
    println!("Received event: {:#?}", event);

    StatusCode::OK
}