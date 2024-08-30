use crate::models::{NewProduct, Product};
use crate::services::ProductService;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::Error;
use std::sync::Arc;

pub async fn create_product(
    State(product_service): State<Arc<ProductService>>,
    Json(product): Json<NewProduct>,
) -> Result<Json<Value>, (StatusCode, String)> {
    match product_service.create_product(product).await {
        Ok(product) => Ok(Json(json!(product))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

pub async fn get_products(
    State(product_service): State<Arc<ProductService>>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    match product_service.get_products().await {
        Ok(products) => Ok(Json(products)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

pub async fn get_one_product(
    State(product_service): State<Arc<ProductService>>,
    Path(id): Path<u64>,
) -> Result<Json<Product>, (StatusCode, String)> {
    match product_service.get_one_product(id).await {
        Ok(product) => Ok(Json(product)),
        Err(Error::RowNotFound) => Err((StatusCode::NOT_FOUND, String::from("Product not found"))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error is: {:?}", err),
        )),
    }
}

pub async fn delete_product(
    State(product_service): State<Arc<ProductService>>,
    Path(id): Path<u64>,
) -> Result<Json<Value>, (StatusCode, String)> {
    match product_service.delete_product(id).await {
        Ok(()) => Ok(Json(json!({"msg": "product deleted successfully"}))),
        Err(Error::RowNotFound) => Err((StatusCode::NOT_FOUND, String::from("Product not found"))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error is: {:?}", err),
        )),
    }
}

pub async fn update_product(
    State(product_service): State<Arc<ProductService>>,
    Path(id): Path<u64>,
    Json(product): Json<Product>,
) -> Result<Json<Value>, (StatusCode, String)> {
    match product_service.update_product(id, product).await {
        Ok(()) => Ok(Json(json!({"msg": "product updated successfully"}))),
        Err(Error::RowNotFound) => Err((StatusCode::NOT_FOUND, String::from("Product not found"))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error is: {:?}", err),
        )),
    }
}
