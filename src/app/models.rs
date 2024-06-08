use sqlx::PgPool;

use crate::app::schemas::{UserSchemaIn, UserSchemaOut};


impl UserSchemaIn {
    pub async fn create_user(&self, pool:&PgPool) {
      sqlx::query!(
        "INSERT INTO app_users(username, email, password) VALUES ($1, $2, $3)",
        &self.username, 
        &self.email,
        &self.password
      )
      .execute(pool)
      .await
      .expect("Failed to create user");

      println!("Created User Successfully")
    }

    pub async fn get_user(email: String, pool:&PgPool) -> Option<UserSchemaOut> {
        sqlx::query_as!(
            UserSchemaOut, "SELECT id, username, email FROM app_users WHERE email = $1",
            &email
        )
        .fetch_optional(pool)
        .await
        .expect("Failed to get user")
    }


    pub async fn get_all_users(pool: &PgPool) -> Vec<UserSchemaOut> {
        sqlx::query_as!(UserSchemaOut, "SELECT id, username, email FROM app_users")
        .fetch_all(pool)
        .await
        .expect("Failed to retrieve all users")
    }
}