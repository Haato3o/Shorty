use serde::{
    Serialize,
    Deserialize
};

use super::metrics::Access;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortenedUrlMetrics {
    pub accesses: Vec<Access>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortenedUrl {
    pub discriminator: String,
    pub redirect_link: String,
    pub owner: String,
    pub expiry_timestamp: i64,
    pub metrics: ShortenedUrlMetrics
}