use serenity::all::{CommandInteraction, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage};

use crate::{DbPool, config, morex};

pub fn register() -> CreateCommand {
    CreateCommand::new("balance")
        .description("View balance.")
        .description_localized("pl", "Wyświetl stan konta.")
        .add_option(
            CreateCommandOption::new(serenity::all::CommandOptionType::User, "member", "Select a member.")
            .name_localized("pl", "osoba")
            .description_localized("pl", "Wybierz osobę")
            .required(false)
        )

}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let a = ctx.data.read().await;
    let pool = a.get::<DbPool>().unwrap();

    morex::account::create_account(&interaction.user.id, &pool).await;
    let balance = morex::economy::get_bank_account(&interaction.user.id, &pool).await;

    let av = interaction.user.avatar_url().unwrap_or_else(|| interaction.user.default_avatar_url());
    

    let embed = CreateEmbed::new()
        .color(config::EMBED_COLOR)
        .author(
            CreateEmbedAuthor::new(format!("Balance {}", &interaction.user.name))
            .icon_url(av)
        )
        .fields(vec![
                ("Coins in the wallet:", format!("{} COIN", balance.wallet), false),
                ("Coins in the bank:", format!("{} COIN", balance.bank), false),
                ("All coins:", format!("{} COIN", balance.bank + balance.wallet), false),
            ]
        )
        .footer(
            CreateEmbedFooter::new(morex::get_version())
        );
        
    let response = CreateInteractionResponseMessage::new()
        .embed(embed);
        // .content(format!("Your wallet: {}\nYour bank: {}", balance.wallet, balance.bank));
    let response = CreateInteractionResponse::Message(response);

    let msg = interaction.create_response(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("Failed to respond to ping command, {e}")
    }
}
