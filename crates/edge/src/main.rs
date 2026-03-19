use axum::{
    routing::post,
    Router,
};

pub mod handlers;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/collect", post( handlers::collect_event));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[ARGUS] Server up and running at port 3000");
    axum::serve(listener, app).await.unwrap();
}
