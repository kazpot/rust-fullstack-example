use crate::models::{NewProduct, Product};
use crate::repository::ProductRepository;

pub struct ProductService {
    repository: ProductRepository,
}

impl ProductService {
    pub fn new(repository: ProductRepository) -> Self {
        Self { repository }
    }

    pub async fn create_product(&self, product: NewProduct) -> Result<NewProduct, String> {
        self.repository.create_product(product).await
    }

    pub async fn get_products(&self) -> Result<Vec<Product>, String> {
        self.repository.get_products().await
    }

    pub async fn get_one_product(&self, id: u64) -> Result<Product, sqlx::Error> {
        self.repository.get_one_product(id).await
    }

    pub async fn delete_product(&self, id: u64) -> Result<(), sqlx::Error> {
        self.repository.delete_product(id).await
    }

    pub async fn update_product(&self, id: u64, product: Product) -> Result<(), sqlx::Error> {
        self.repository.update_product(id, product).await
    }
}
