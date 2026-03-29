use chrono::{Utc};
use serenity::{all::UserId};
use sqlx::PgPool;

pub async fn create_account(user_id: &UserId, pool: &PgPool) {
    let timestamp = chrono::Local::now();
    let uid: i64 = user_id.get() as i64;
    sqlx::query!("INSERT INTO users (user_id, wallet, bank, hp, level, xp, total_xp, version, timestamp) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT (user_id) DO NOTHING", uid, 0, 0, 100, 0, 0, 0, "0.1.0", timestamp)
        .execute(pool)
        .await
        .expect("It failed");
}

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

pub async fn fetch_account(user_id: &UserId, pool: &PgPool) -> User {
    let uid = user_id.get() as i64;
    let user: User = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", uid)
        .fetch_one(pool)
        .await
        .expect("Failed to fetch the account");
    user
}
