use crate::domain::entities;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Model {
    pub id: i64,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub username: String,
    pub password: String,
}

impl From<Model> for entities::User {
    fn from(val: Model) -> Self {
        entities::User {
            id: val.id as i32,
            first_name: val.first_name,
            middle_name: val.middle_name,
            last_name: val.last_name,
            username: val.username,
            password: val.password,
        }
    }
}
