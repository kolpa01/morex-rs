use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub struct DataBaseManager {
   pool: PgPool
}

impl DataBaseManager {
   pub async fn new() -> Self {
      let pool = PgPoolOptions::new()
          .max_connections(1)
          .connect("postgres://postgres:password@localhost/MorexDB")
          .await
          .expect("Failed initialize DataBaseManager!");

      let manager = Self { pool };
      manager.create_tables().await;

      println!("DataBaseManager initialized!");
      manager
   }

   pub async fn create_tables(&self) {
      sqlx::query(r#"CREATE TABLE IF NOT EXISTS users (user_id bigint PRIMARY KEY, wallet bigint NOT NULL, bank bigint NOT NULL, hp bigint NOT NULL, level bigint NOT NULL, xp bigint NOT NULL, total_xp bigint NOT NULL, version varchar(8) NOT NULL, timestamp timestamptz(0) NOT NULL)"#)
          .execute(&self.pool)
          .await
          .expect("Failed to create tables!");
   }

   pub fn get_pool(&self) -> &PgPool {
      &self.pool
   }
}