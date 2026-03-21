use axum::{extract::{Json, State}, http::StatusCode};
use argus_core::models::Event;
use argus_core::queue::EventProducer;

pub async fn collect_event(
    State(producer): State<EventProducer>,
    Json(event): Json<Event>,
) -> StatusCode {

    match producer.push(event).await {
        Ok(_) => {
            StatusCode::OK
        }
        Err(e) => {
            // The only reason this fails is if the queue crashed or closed.
            eprintln!("CRITICAL ERROR: Failed to enqueue event: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}