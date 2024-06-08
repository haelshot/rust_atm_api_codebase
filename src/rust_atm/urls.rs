use axum::routing::{get, post, Router};
use crate::app::controllers::auth::{signup, get_all_users};


pub async fn app_router() -> Router {
    let router = Router::new()
    .route("/check_db_status", get("Status Ok1"))
    .route("/signup", post(signup))
    .route("/get_all_users", post(get_all_users));

    return router;
}

