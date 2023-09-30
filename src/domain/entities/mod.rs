use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}
