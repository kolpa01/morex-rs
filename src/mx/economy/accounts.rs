use chrono::Utc;
use serenity::all::{UserId};
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct User {
   pub user_id: i64,
   pub wallet: i64,
   pub bank: i64,
   pub hp: i64,
   pub level: i64,
   pub xp: i64,
   pub total_xp: i64,
   pub version: String,
   pub timestamp: chrono::DateTime<Utc>
}

pub struct Accounts;
impl Accounts {
   pub async fn create_account(user_id: &UserId, pool: &PgPool) {
      let timestamp = chrono::Local::now();
      let uid = user_id.get() as i64;
      sqlx::query("INSERT INTO users (user_id, wallet, bank, hp, level, xp, total_xp, version, timestamp) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
          .bind(uid)
          .bind(0)
          .bind(0)
          .bind(100)
          .bind(0)
          .bind(0)
          .bind(0)
          .bind("0.1.0")
          .bind(timestamp)
          .execute(pool)
          .await
          .expect("Failed create account!");
   }

   pub async fn delete_account(user_id: &UserId, pool: &PgPool) {
      let uid = user_id.get() as i64;
      sqlx::query("DELETE FROM users WHERE user_id = $1")
          .bind(uid)
          .execute(pool)
          .await
          .expect("Failed delete account!");
   }

   pub async fn account_exists(user_id: &UserId, pool: &PgPool) -> Result<bool, sqlx::Error> {
      let uid = user_id.get() as i64;
      let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE user_id = $1)")
          .bind(uid)
          .fetch_one(pool)
          .await?;
      Ok(exists.0)
   }

   pub async fn fetch_account(user_id: &UserId, pool: &PgPool) -> User {
      let uid = user_id.get() as i64;
      let user: User = sqlx::query_as("SELECT * FROM users WHERE user_id = $1")
          .bind(uid)
          .fetch_one(pool)
          .await
          .expect("Failed to fetch the account");
      user
   }
}