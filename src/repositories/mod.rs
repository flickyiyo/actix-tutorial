pub mod memory_repository;
use crate::models::{User, CustomData};

pub use memory_repository::MemoryRepository;

pub trait Repository {
    fn get_users(&self, user_id: &uuid::Uuid) -> Result<User, String>;
}
