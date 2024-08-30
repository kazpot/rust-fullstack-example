use crate::models::{NewProduct, Product};
use async_trait::async_trait;
use sqlx::{MySql, Pool};

#[async_trait]
pub trait ProductRepositoryTrait: Send + Sync {
    async fn create_product(&self, product: NewProduct) -> Result<NewProduct, String>;
    async fn get_products(&self) -> Result<Vec<Product>, String>;
    async fn get_one_product(&self, id: u64) -> Result<Product, sqlx::Error>;
    async fn delete_product(&self, id: u64) -> Result<(), sqlx::Error>;
    async fn update_product(&self, id: u64, product: Product) -> Result<(), sqlx::Error>;
}

pub struct ProductRepository {
    pool: Pool<MySql>,
}

impl ProductRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepositoryTrait for ProductRepository {
    async fn create_product(&self, product: NewProduct) -> Result<NewProduct, String> {
        sqlx::query("INSERT INTO products (name, price) VALUES (?, ?)")
            .bind(&product.name)
            .bind(&product.price)
            .execute(&self.pool)
            .await
            .map_err(|err| format!("Error is: {:?}", err))?;
        Ok(product)
    }

    async fn get_products(&self) -> Result<Vec<Product>, String> {
        let products = sqlx::query_as("SELECT * FROM products")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| format!("Error is {:?}", err))?;
        Ok(products)
    }

    async fn get_one_product(&self, id: u64) -> Result<Product, sqlx::Error> {
        let product = sqlx::query_as("SELECT * FROM products WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| err)?;
        Ok(product)
    }

    async fn delete_product(&self, id: u64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| err)?;
        Ok(())
    }

    async fn update_product(&self, id: u64, product: Product) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE products SET name = ?, price = ? WHERE id = ?")
            .bind(&product.name)
            .bind(&product.price)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| err)?;
        Ok(())
    }
}
