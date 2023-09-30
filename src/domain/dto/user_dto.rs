use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUserDTO {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub username: String,
    pub password: String,
}
