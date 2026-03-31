use serenity::all::{ActivityData, Context, EventHandler, Interaction, Ready};
use serenity::async_trait;
use crate::mx::managers::commands_manager::CommandsManager;

pub struct Handler {
   pub(crate) commands_manager: CommandsManager,
}

#[async_trait]
impl EventHandler for Handler {
   async fn ready(&self, ctx: Context, ready: Ready) {
      println!("Logged in {}", ready.user.name);
      ctx.set_activity(Some(ActivityData::watching("Slime Valley")));
      self.commands_manager.register_commands(&ctx).await;
   }

   async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
      self.commands_manager.handle_commands(&ctx, &interaction).await;
   }
}