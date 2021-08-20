pub mod link;
use std::collections::HashMap;

use domain::{errors::Error, repository::Connectable};
use mongodb::{Client, Database, options::ClientOptions};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct Mongo {
    connection: Database
}

#[async_trait]
impl Connectable for Mongo {
    async fn connect(credentials: HashMap<String, String>) -> Result<Box<Self>, Error> {

        let uri = credentials.get("mongo_uri").expect("mongo_uri cannot be empty");
        let database_name = credentials.get("database_name").expect("database_name cannot be empty");

        match ClientOptions::parse(&uri).await {
            Ok(options) => match Client::with_options(options) {
                Ok(client) => Ok(
                    Box::from(Mongo {
                        connection: client.database(&database_name)
                    })
                ),
                Err(_) => Err(Error::FailedToConnect),
            },
            Err(_) => Err(Error::FailedToConnect),
        }
    }
}