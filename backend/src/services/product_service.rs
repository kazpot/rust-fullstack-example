use crate::models::{NewProduct, Product};
use crate::repository::ProductRepository;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

pub struct ProductService {
    repository: ProductRepository,
}

impl ProductService {
    pub fn new(repository: ProductRepository) -> Self {
        Self { repository }
    }

    pub async fn create_product(
        &self,
        product: NewProduct,
    ) -> Result<Json<Value>, (StatusCode, String)> {
        self.repository.create_product(product).await
    }

    pub async fn get_products(&self) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
        self.repository.get_products().await
    }

    pub async fn get_one_product(&self, id: u64) -> Result<Json<Product>, (StatusCode, String)> {
        self.repository.get_one_product(id).await
    }

    pub async fn delete_product(&self, id: u64) -> Result<Json<Value>, (StatusCode, String)> {
        self.repository.delete_product(id).await
    }

    pub async fn update_product(
        &self,
        id: u64,
        product: Product,
    ) -> Result<Json<Value>, (StatusCode, String)> {
        self.repository.update_product(id, product).await
    }
}
