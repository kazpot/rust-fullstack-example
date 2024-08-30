use async_trait::async_trait;
use backend::models::{NewProduct, Product};
use backend::repository::product_repository::ProductRepositoryTrait;

struct MockProductRepository;

#[async_trait]
impl ProductRepositoryTrait for MockProductRepository {
    async fn create_product(&self, product: NewProduct) -> Result<NewProduct, String> {
        Ok(product)
    }

    async fn get_products(&self) -> Result<Vec<Product>, String> {
        Ok(vec![
            Product {
                id: 1,
                name: "Product 1".to_string(),
                price: 100,
            },
            Product {
                id: 2,
                name: "Product 2".to_string(),
                price: 200,
            },
        ])
    }

    async fn get_one_product(&self, id: u64) -> Result<Product, sqlx::Error> {
        match id {
            1 => Ok(Product {
                id,
                name: "Product 1".to_string(),
                price: 100,
            }),
            _ => Err(sqlx::Error::RowNotFound),
        }
    }

    async fn delete_product(&self, id: u64) -> Result<(), sqlx::Error> {
        if id == 1 {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    async fn update_product(&self, id: u64, _product: Product) -> Result<(), sqlx::Error> {
        if id == 1 {
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backend::models::NewProduct;
    use backend::services::ProductService;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_product() {
        let mock_repo = Arc::new(MockProductRepository);
        let service = ProductService::new(mock_repo);

        let new_product = NewProduct {
            name: "Test Product".to_string(),
            price: 100,
        };

        let result = service.create_product(new_product.clone()).await;

        assert!(result.is_ok());
        let product = result.unwrap();
        assert_eq!(product.name, new_product.name);
        assert_eq!(product.price, new_product.price);
    }

    #[tokio::test]
    async fn test_get_one_product_success() {
        let mock_repo = Arc::new(MockProductRepository) as Arc<dyn ProductRepositoryTrait>;
        let service = ProductService::new(mock_repo);

        let product = service.get_one_product(1).await;

        assert!(product.is_ok());
        let product = product.unwrap();
        assert_eq!(product.name, "Product 1");
        assert_eq!(product.price, 100);
    }

    #[tokio::test]
    async fn test_get_one_product_not_found() {
        let mock_repo = Arc::new(MockProductRepository) as Arc<dyn ProductRepositoryTrait>;
        let service = ProductService::new(mock_repo);

        let result = service.get_one_product(999).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_product() {
        let mock_repo = Arc::new(MockProductRepository) as Arc<dyn ProductRepositoryTrait>;
        let service = ProductService::new(mock_repo);

        let result = service.delete_product(1).await;

        assert!(result.is_ok());

        let result = service.delete_product(999).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_product() {
        let mock_repo = Arc::new(MockProductRepository) as Arc<dyn ProductRepositoryTrait>;
        let service = ProductService::new(mock_repo);

        let updated_product = Product {
            id: 1,
            name: "Updated Product".to_string(),
            price: 150,
        };

        let result = service.update_product(1, updated_product.clone()).await;

        assert!(result.is_ok());

        let result = service.update_product(999, updated_product.clone()).await;

        assert!(result.is_err());
    }
}
