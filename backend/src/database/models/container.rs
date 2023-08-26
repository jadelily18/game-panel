use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub label: String,
    pub port: i32
}

