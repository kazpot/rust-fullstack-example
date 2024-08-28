use crate::models::{NewProduct, Product};
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use sqlx::{MySql, Pool};

pub struct ProductRepository {
    pool: Pool<MySql>,
}

impl ProductRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    pub async fn create_product(
        &self,
        product: NewProduct,
    ) -> Result<Json<Value>, (StatusCode, String)> {
        sqlx::query("INSERT INTO products (name, price) VALUES (?, ?)")
            .bind(&product.name)
            .bind(&product.price)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error is: {:?}", err),
                )
            })?;
        Ok(Json(json!(product)))
    }

    pub async fn get_products(&self) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
        let products = sqlx::query_as("SELECT * FROM products")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error is {:?}", err),
                )
            })?;
        Ok(Json(products))
    }

    pub async fn get_one_product(&self, id: u64) -> Result<Json<Product>, (StatusCode, String)> {
        let product = sqlx::query_as("SELECT * FROM products WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
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

    pub async fn delete_product(&self, id: u64) -> Result<Json<Value>, (StatusCode, String)> {
        sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
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
        &self,
        id: u64,
        product: Product,
    ) -> Result<Json<Value>, (StatusCode, String)> {
        sqlx::query("UPDATE products SET name = ?, price = ? WHERE id = ?")
            .bind(&product.name)
            .bind(&product.price)
            .bind(id)
            .execute(&self.pool)
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
}
