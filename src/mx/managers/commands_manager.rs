use serenity::all::{Command, Interaction};
use serenity::client::Context;
use sqlx::PgPool;
use crate::mx::commands;

pub struct CommandsManager {
   pool: PgPool
}

impl CommandsManager {
   pub fn new(pool: &PgPool) -> Self {
      println!("CommandsManager initialized!");
      Self { pool: pool.clone() }
   }

   pub async fn register_commands(&self, ctx: &Context) {
      Command::set_global_commands(&ctx.http,
         vec![
            commands::ping::Ping::register(),
            commands::balance::Balance::register()
         ]
      ).await.expect("Failed to register commands!");
   }

   pub async fn handle_commands(&self, ctx: &Context, interaction: &Interaction) {
      if let Interaction::Command(cmd) = interaction {
         match cmd.data.name.as_str() {
            "ping" => { commands::ping::Ping::handle(&ctx, &cmd).await; }
            "balance" => { commands::balance::Balance::handle(&self.pool, &ctx, &cmd).await }
            _ => {}
         }
      }
   }
}