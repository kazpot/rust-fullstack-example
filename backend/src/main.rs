mod handlers;
mod models;
mod repository;
mod services;

use crate::repository::product_repository::ProductRepositoryTrait;
use crate::repository::ProductRepository;
use crate::services::ProductService;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use sqlx::{MySql, Pool};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

async fn api_key_check(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = bearer.token();

    if api_key == "your_secret_api_key" {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool: Pool<MySql> = sqlx::MySqlPool::connect(&database_url)
        .await
        .expect("Error with pool connection");

    let product_repository =
        Arc::new(ProductRepository::new(pool)) as Arc<dyn ProductRepositoryTrait>;
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
        .layer(middleware::from_fn(api_key_check))
        .layer(cors);

    tracing::info!("listening on port {}", "0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Home"
}
