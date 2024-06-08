
use axum::{
    http::StatusCode, response::IntoResponse, Json
};

use crate::app::schemas::{ReturnResponse, UserSchemaIn};
use crate::rust_atm::settings::app_state;


pub async fn signup(
    Json(fields): Json<UserSchemaIn>,
) -> impl IntoResponse {
    // Validate the fields
    if fields.username.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Username is required").into_response();
    } else if fields.password.len() < 6 {
        return (StatusCode::BAD_REQUEST, "Password must be at least 6 characters long").into_response();
    } else if !fields.email.contains('@') {
        return (StatusCode::BAD_REQUEST, "Invalid email address").into_response();
    }

    let state = app_state().await;

    fields.create_user(&state.pool).await;

    let current_user = UserSchemaIn::get_user(fields.email.clone(), &state.pool).await;

    let response = ReturnResponse {
        is_success: true,
        message: "User Created successfully".to_string(),
        payload: current_user
    };

    (StatusCode::CREATED, Json(response)).into_response()
}


pub async fn get_all_users() -> impl IntoResponse {
    let state = app_state().await;
    let users = UserSchemaIn::get_all_users(&state.pool).await;
    let response = ReturnResponse {
        is_success: true,
        message: "Users retrieved successfully".to_string(),
        payload: users
    };
    (StatusCode::CREATED, Json(response)).into_response()
}