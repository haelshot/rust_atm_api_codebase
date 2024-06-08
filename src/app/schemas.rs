use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UserSchemaIn {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct UserSchemaOut {
    pub id: i64,
    pub username: String,
    pub email: String
}

#[derive(Serialize)]
pub struct ReturnResponse<T> {
    pub is_success: bool,
    pub message: String,
    pub payload: T
}