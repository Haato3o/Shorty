use serde::{
    Deserialize,
    Serialize
};

pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub token: String,
    pub created_at: i64
}