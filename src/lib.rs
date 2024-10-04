// register global module distribution
pub mod service;

// imports
use axum::{middleware, Router};

pub async fn run() {
    let app = Router::new()
        .nest("/api/", service::routes::user_route::user_routes().await)
        .route_layer(middleware::from_fn(
            service::middlewares::route_logger::simple_log,
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
