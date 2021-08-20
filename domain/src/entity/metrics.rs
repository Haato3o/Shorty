use std::u64;

use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Access {
    pub timestamp: i64,
    pub count: u64
}