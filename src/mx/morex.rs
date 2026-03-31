use serenity::all::GatewayIntents;
use serenity::Client;

use crate::mx::managers::commands_manager::CommandsManager;
use crate::mx::managers::data_base_manager::DataBaseManager;
use crate::mx::handler::Handler;

pub struct Morex {
   token: String
}

impl Morex {
   pub fn new() -> Self {
      Self { token: dotenv::var("TOKEN").unwrap() }
   }

   pub async fn run(&self) {
      let data_base_manager = DataBaseManager::new().await;
      let commands_manager = CommandsManager::new(data_base_manager.get_pool());
      let handler = Handler {
         commands_manager
      };

      let mut client = Client::builder(&self.token, GatewayIntents::all())
          .event_handler(handler)
          .await
          .expect("Failed to create client!");

      if let Err(e) = client.start().await {
         println!("Failed to initialize client: {e}")
      }
   }
}