use argus_core::queue::create_ingestion_queue;
use axum::{
    routing::post,
    Router,
};

pub mod handlers;

#[tokio::main]
async fn main() {

    let (producer, mut consumer) = create_ingestion_queue(10_000);

    tokio::spawn(async move {
        println!("[BACKGROUND WAL] worker started...");

        'wal_worker: loop {
            let batch = consumer.flush_batch(500).await;

            if !batch.is_empty() {

                println!("Worker flushed a batch of {} events to storage.", batch.len());
            } else {
                break 'wal_worker;
            }
        }
    });


    let app = Router::new()
        .route("/collect", post( handlers::collect_event))
        .with_state(producer);



    println!("[ARGUS] Server up and running at port 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
