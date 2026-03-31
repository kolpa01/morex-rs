use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage};
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
      let av = interaction.user.avatar_url().unwrap_or_else(|| interaction.user.default_avatar_url());

      let embed = CreateEmbed::new()
          .color((b'\xce', b'\x4d', b'\xd5'))
          .author(
             CreateEmbedAuthor::new(format!("Balance {}", &interaction.user.name)).icon_url(av)
          )
          .fields(vec![
             ("Coins in the wallet:", format!("{} COIN", balance.wallet), false),
             ("Coins in the bank:", format!("{} COIN", balance.bank), false),
             ("All coins:", format!("{} COIN", balance.bank + balance.wallet), false)
          ])
          .footer(
             CreateEmbedFooter::new("Version 0.1.0")
          );

      let msg = interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
         CreateInteractionResponseMessage::new().embed(embed)
      )).await;

      if let Err(e) = msg {
         println!("Failed to respond to balance command, {e}")
      }
   }
}