use async_trait::async_trait;
use dyn_clone::{
    DynClone,
    clone_trait_object
};
use crate::{entity::user::User, errors::Error};

#[async_trait]
pub trait Reader: DynClone {
    async fn get_user(&self, username: String) -> Result<Option<User>, Error>;
}

#[async_trait]
pub trait Writer: DynClone {
    
    async fn create_user(
        &self, 
        username: String,
        password: String,
        email: String
    ) -> Result<bool, Error>;

}

pub trait UserRepository: DynClone + Writer + Reader {}

clone_trait_object!(UserRepository);