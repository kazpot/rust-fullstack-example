use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::MySqlPool;

#[derive(Serialize, Deserialize)]
pub struct NewProduct {
    name: String,
    price: i32,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    id: u64,
    name: String,
    price: i32,
}

pub async fn create_product(
    State(pool): State<MySqlPool>,
    Json(product): Json<NewProduct>,
) -> Result<Json<Value>, (StatusCode, String)> {
    sqlx::query("INSERT INTO products (name, price) VALUES (?, ?)")
        .bind(&product.name)
        .bind(&product.price)
        .execute(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is: {:?}", err),
            )
        })?;
    Ok(Json(json!(product)))
}

pub async fn get_products(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    let products = sqlx::query_as("SELECT * FROM products")
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {:?}", err),
            )
        })?;
    Ok(Json(products))
}

pub async fn get_one_product(
    State(pool): State<MySqlPool>,
    Path(id): Path<u64>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let product = sqlx::query_as("SELECT * FROM products WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is: {:?}", err)),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is: {:?}", err),
            ),
        })?;
    Ok(Json(product))
}

pub async fn delete_product(
    State(pool): State<MySqlPool>,
    Path(id): Path<u64>,
) -> Result<Json<Value>, (StatusCode, String)> {
    sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is: {:?}", err)),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is: {:?}", err),
            ),
        })?;
    Ok(Json(json!({"msg": "Product deleted successfully"})))
}

pub async fn update_product(
    State(pool): State<MySqlPool>,
    Path(id): Path<u64>,
    Json(product): Json<Product>,
) -> Result<Json<Value>, (StatusCode, String)> {
    sqlx::query("UPDATE products SET name = ?, price = ? WHERE id = ?")
        .bind(&product.name)
        .bind(&product.price)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is: {:?}", err)),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is: {:?}", err),
            ),
        })?;
    Ok(Json(json!({"msg": "Product updated successfully"})))
}
