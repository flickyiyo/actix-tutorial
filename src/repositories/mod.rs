pub mod memory_repository;
use std::{ops::Deref, sync::Arc};

use crate::models::{User};

pub use memory_repository::MemoryRepository;

pub trait Repository: Send + Sync + 'static {
    fn get_users(&self, user_id: &uuid::Uuid) -> Result<User, String>;
}

pub struct RepositoryInjector(Box<dyn Repository>);

impl RepositoryInjector {
    pub fn new(repo: impl Repository) -> Self {
        Self(Box::new(repo))
    }

    pub fn new_shared(repo: impl Repository) -> Arc<Self> {
        Arc::new(Self::new(repo))
    }
}

impl Deref for RepositoryInjector {
    type Target = dyn Repository;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}