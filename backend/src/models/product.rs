use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub price: i32,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: i32,
}
