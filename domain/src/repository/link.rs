use async_trait::async_trait;
use dyn_clone::{
    DynClone,
    clone_trait_object
};
use crate::{
    entity::link::ShortenedUrl,
    errors::Error
};

#[async_trait]
pub trait Reader: DynClone {
    async fn get_link(&self, discriminator: String) -> Result<Option<ShortenedUrl>, Error>;
    async fn get_links(&self, user_token: String) -> Result<Option<Vec<ShortenedUrl>>, Error>;
}

#[async_trait]
pub trait Writer: DynClone {
    async fn create_link(&self, link: String) -> Result<ShortenedUrl, Error>;
    async fn delete_link(&self, user_token: String, discriminator: String) -> Result<bool, Error>;
}

#[async_trait]
pub trait LinkRepository: DynClone + Reader + Writer {}

clone_trait_object!(LinkRepository);