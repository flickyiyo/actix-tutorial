use crate::models::User;
use crate::repositories::Repository;
pub struct MemoryRepository {
    users: Vec<User>,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: vec![User::new("Rob".to_string(), (2000, 12, 12))],
        }
    }
}

impl Repository for MemoryRepository {
    fn get_users(&self, user_id: &uuid::Uuid) -> Result<User, String> {
        self.users
            .iter()
            .find(|u| &u.id == user_id)
            .cloned()
            .ok_or_else(|| "User Not Found".to_string())
    }
}
