use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub age: i16,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser{
    pub username: String,
    pub password: String, 
    pub age: i16,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser{
    pub username: Option<String>,
    pub age: Option<i16>,
}