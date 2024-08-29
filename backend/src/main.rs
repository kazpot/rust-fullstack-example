mod handlers;
mod models;
mod repository;
mod services;

use crate::repository::ProductRepository;
use crate::services::ProductService;
use axum::{routing::get, Router};
use sqlx::{MySql, Pool};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    dotenv::dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool: Pool<MySql> = sqlx::MySqlPool::connect(&database_url)
        .await
        .expect("Error with pool connection");

    let product_repository = ProductRepository::new(pool);
    let product_service = Arc::new(ProductService::new(product_repository));

    let app = Router::new()
        .route("/home", get(root))
        .route(
            "/api/products",
            get(handlers::get_products).post(handlers::create_product),
        )
        .route(
            "/api/products/:id",
            get(handlers::get_one_product)
                .delete(handlers::delete_product)
                .put(handlers::update_product),
        )
        .with_state(product_service)
        .layer(cors);

    tracing::info!("listening on port {}", "0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Home"
}
