use std::u64;

use domain::{entity::{link::ShortenedUrlMetrics, metrics::Access}, errors::Error, repository::{
    link::{
        LinkRepository,
        Reader as LinkReader,
        Writer as LinkWriter
    }
}};
use async_trait::async_trait;
use domain::entity::{
    link::ShortenedUrl
};
use mongodb::bson::{DateTime, doc, oid::ObjectId};
use crate::infrastructure::mongodb::Mongo;
use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
struct LinkAccessModel {
    pub timestamp: DateTime,
    pub count: u64
}

impl Into<Access> for LinkAccessModel {
    fn into(self) -> Access {
        Access {
            timestamp: self.timestamp.timestamp(),
            count: self.count
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct LinkMetricsModel {
    pub accesses: Vec<LinkAccessModel>
}

impl Into<ShortenedUrlMetrics> for LinkMetricsModel {
    fn into(self) -> ShortenedUrlMetrics {
        let mut intoAccesses: Vec<Access> = vec!();

        for access in self.accesses.into_iter() {
            intoAccesses.push(access.into())
        }

        ShortenedUrlMetrics {
            accesses: intoAccesses
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct LinkModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub discriminator: String,
    pub redirect_link: String,
    pub owner: String,
    pub expiry_timestamp: i64,
    pub metrics: LinkMetricsModel
}

impl Into<ShortenedUrl> for LinkModel {
    fn into(self) -> ShortenedUrl {
        ShortenedUrl {
            discriminator: self.discriminator,
            redirect_link: self.redirect_link,
            owner: self.owner,
            expiry_timestamp: self.expiry_timestamp,
            metrics: self.metrics.into()
        }
    }
}

#[async_trait]
impl LinkReader for Mongo {
    async fn get_link(&self, discriminator: String) -> Result<Option<ShortenedUrl>, Error> {
        let collection = self.connection.collection("links");

        let query = doc! {
            "discriminator": discriminator
        };

        match collection.find_one(query, None).await {
            Ok(document) => match document {
                Some(doc) => {
                    let model = mongodb::bson::from_document::<LinkModel>(doc).unwrap();

                    Ok(
                        Some(model.into())
                    )
                },
                None => Ok(None),
            },
            Err(_) => Err(Error::NotFound),
        }
    }

    async fn get_links(&self, user_token: String) -> Result<Option<Vec<ShortenedUrl>>, Error> {
        // TODO: Implement this
        Ok(None)
    }
}

#[async_trait]
impl LinkWriter for Mongo {
    async fn create_link(&self, link: String) -> Result<ShortenedUrl, Error> {
        todo!()
    }

    async fn delete_link(&self, user_token: String, discriminator: String) -> Result<bool, Error> {
        Ok(false)
    }
}

impl LinkRepository for Mongo {}