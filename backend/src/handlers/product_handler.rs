use crate::models::{NewProduct, Product};
use crate::services::ProductService;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;
use std::sync::Arc;

pub async fn create_product(
    State(product_service): State<Arc<ProductService>>,
    Json(product): Json<NewProduct>,
) -> Result<Json<Value>, (StatusCode, String)> {
    product_service.create_product(product).await
}

pub async fn get_products(
    State(product_service): State<Arc<ProductService>>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    product_service.get_products().await
}

pub async fn get_one_product(
    State(product_service): State<Arc<ProductService>>,
    Path(id): Path<u64>,
) -> Result<Json<Product>, (StatusCode, String)> {
    product_service.get_one_product(id).await
}

pub async fn delete_product(
    State(product_service): State<Arc<ProductService>>,
    Path(id): Path<u64>,
) -> Result<Json<Value>, (StatusCode, String)> {
    product_service.delete_product(id).await
}

pub async fn update_product(
    State(product_service): State<Arc<ProductService>>,
    Path(id): Path<u64>,
    Json(product): Json<Product>,
) -> Result<Json<Value>, (StatusCode, String)> {
    product_service.update_product(id, product).await
}
