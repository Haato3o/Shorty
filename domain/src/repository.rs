pub mod link;
pub mod user;
use std::{collections::HashMap};

use async_trait::async_trait;

use crate::errors::Error;

use self::{link::LinkRepository, user::UserRepository};

#[derive(Clone)]
pub struct Repository {
    pub link: Box<dyn LinkRepository + Send>,
    pub user: Box<dyn UserRepository + Send>
}

#[async_trait]
pub trait Connectable {
    async fn connect(credentials: HashMap<String, String>) -> Result<Box<Self>, Error>;
}