use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use sqlx::PgPool;

use crate::mx::economy::balance;
use crate::mx::economy::accounts::Accounts;

pub struct Balance;
impl Balance {
   pub fn register() -> CreateCommand {
      CreateCommand::new("balance")
          .description("Balance")
   }

   pub async fn handle(pool: &PgPool, ctx: &Context, interaction: &CommandInteraction) {
      let exists = Accounts::account_exists(&interaction.user.id, pool).await.unwrap_or(false);
      if !exists {
         Accounts::create_account(&interaction.user.id, &pool).await
      }

      let balance: balance::BankAccount = balance::Balance::get_bank_account(&interaction.user.id, &pool).await;
      let msg = interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
         CreateInteractionResponseMessage::new().content(format!("Your wallet: {}\nYour bank: {}", balance.wallet, balance.bank))
      )).await;

      if let Err(e) = msg {
         println!("Failed to respond to balance command, {e}")
      }
   }
}