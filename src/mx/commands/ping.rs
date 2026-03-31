use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};

pub struct Ping;
impl Ping {
   pub fn register() -> CreateCommand {
      CreateCommand::new("ping")
          .description("Test command")
   }

   pub async fn handle(ctx: &Context, interaction: &CommandInteraction) {
      let msg = interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
         CreateInteractionResponseMessage::new().content("Pong!")
      )).await;

      if let Err(e) = msg {
         println!("Failed to respond to ping command, {e}")
      }
   }
}