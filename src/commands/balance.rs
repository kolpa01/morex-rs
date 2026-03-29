use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};

use crate::{DbPool, morex};

pub fn register() -> CreateCommand {
    CreateCommand::new("balance")
        .description("Bal")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let a = ctx.data.read().await;
    let pool = a.get::<DbPool>().unwrap();

    morex::account::create_account(&interaction.user.id, &pool).await;
    let balance = morex::economy::get_bank_account(&interaction.user.id, &pool).await;

    let response = CreateInteractionResponseMessage::new().content(format!("Your wallet: {}\nYour bank: {}", balance.wallet, balance.bank));
    let response = CreateInteractionResponse::Message(response);

    let msg = interaction.create_response(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("Failed to respond to ping command, {e}")
    }
}
