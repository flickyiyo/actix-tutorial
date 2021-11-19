use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub birth_date: chrono::NaiveDate,
    pub custom_data: CustomData,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl User {
    pub fn new(name: String, birth_date_ymd: (i32, u32, u32)) -> Self {
        let (year, month, day) = birth_date_ymd;
        let id = Uuid::parse_str("488ed7c3-68c6-4936-a91c-128be627988b").unwrap();
        Self {
            id, // uuid::Uuid::new_v4(),
            name,
            custom_data: CustomData { random: 1 },
            birth_date: chrono::NaiveDate::from_ymd(year, month, day),
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomData {
    pub random: u32,
}
