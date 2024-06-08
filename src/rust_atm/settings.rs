use std::{env, sync::Arc};

use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct AppStateInner {
   pub pool: PgPool,
}


impl AppStateInner {
    pub async fn new() -> Self {
        dotenvy::dotenv().expect("Could not load the .env file!");
        let database_url =
            env::var("DATABASE_URL").expect("The environment variable DATABASE_URL is missing!");

        let pool = PgPoolOptions::new()
            .connect(&database_url)
            .await
            .expect("Failed to connect to the database!");

        Self { pool }
    }
}

pub async fn app_state() -> Arc<AppStateInner> {
    let state = Arc::new(AppStateInner::new().await);
    state
}


// type AppState = Arc<AppStateInner>;