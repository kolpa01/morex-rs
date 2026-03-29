use serenity::{all::{ActivityData, ClientBuilder, Command, Context, EventHandler, GatewayIntents, Interaction, Ready}, async_trait, prelude::TypeMapKey};
use sqlx::{PgPool, postgres::PgPoolOptions};

pub mod commands;
pub mod morex;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Logged in {}", ready.user.name);

        ctx.set_activity(Some(ActivityData::watching("Rusty Valley")));

        let commands = Command::set_global_commands(&ctx.http,
            vec![
                commands::ping::register(),
                commands::balance::register(),
            ]
        ).await.unwrap();

        println!("Loaded commands: {}", commands.into_iter().map(|c| c.name).collect::<Vec<String>>().join("\n"));
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            match cmd.data.name.as_str() {
                "ping" => {
                    commands::ping::run(&ctx, &cmd).await;
                }
                "balance" => {
                    commands::balance::run(&ctx, &cmd).await;
                }
                _ => {}
            }
        }
    }
}

struct DbPool;

impl TypeMapKey for DbPool {
    type Value = PgPool;
}

#[tokio::main]
async fn main() {
    let db_url = dotenv::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::query!("CREATE TABLE IF NOT EXISTS users (user_id bigint PRIMARY KEY, wallet bigint NOT NULL, bank bigint NOT NULL, hp bigint NOT NULL, level bigint NOT NULL, xp bigint NOT NULL, total_xp bigint NOT NULL, version varchar(8) NOT NULL, timestamp timestamptz(0) NOT NULL)")
        .execute(&pool)
        .await;

    let token = dotenv::var("TOKEN").unwrap();
    let intents = GatewayIntents::all();
    let mut client = ClientBuilder::new(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    client.data.write().await.insert::<DbPool>(pool);

    if let Err(e) = client.start().await {
        println!("Failed to initialize client: {e}");
    }
}
