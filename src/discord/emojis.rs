use serde::Deserialize;
use serde_json::{Map, Value};
use serenity::{all::{Context, Emoji, EmojiId}, prelude::TypeMapKey};
use sqlx::{PgPool};
use base64::{self, Engine};
use image::{DynamicImage, imageops::FilterType};
use std::{fs, io::Cursor};
use sha2::{self, Digest, Sha256};
use hex;

struct EmojiData {
    path: String,
    emoji_id: i64,
    checksum: String,
}

pub struct EmojiHandler {
    pub ui_emojis: Option<UiEmojis>
}

impl EmojiHandler {
    fn new() -> Self {
        Self {
            ui_emojis: None
        }
    }

    fn set_ui_emojis(&mut self,  ui_emojis: UiEmojis) {
        self.ui_emojis = Some(ui_emojis);
    }
}

impl TypeMapKey for EmojiHandler {
    type Value = EmojiHandler;
}

#[derive(Deserialize, Debug)]
pub struct UiEmojis {
    pub coin: String
}

async fn add_emoji(name: &str, img: &DynamicImage, ctx: &Context) -> Emoji {
    let upscaled = img.resize(img.width() * 10, img.height() * 10, FilterType::Nearest);
    let mut buf = Cursor::new(vec![]);
    upscaled.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write");

    let img_encoded = base64::engine::general_purpose::STANDARD.encode(&buf.into_inner());
    let img_with_that = format!("data:image/png;base64,{}", &img_encoded);

    ctx.create_application_emoji(name, &img_with_that).await.expect("Failed to upload emoji")
}

async fn handle_emoji(pool: &PgPool, ctx: &Context, name: &str, path: &str) -> Emoji {
    let emoji_data = sqlx::query_as!(EmojiData, "SELECT * FROM emojis WHERE path = $1", path)
        .fetch_optional(pool)
        .await
        .expect("Wow something failed");

    let img = image::ImageReader::open(path).unwrap().decode().unwrap();
    let data_in_bytes = &img.clone().into_bytes();
    let hash = Sha256::digest(data_in_bytes);
    let hash_string = hex::encode(hash);

    match emoji_data {
        Some(emoji_data) => {
            if emoji_data.checksum == hash_string {
                ctx.get_application_emoji(EmojiId::new(emoji_data.emoji_id as u64)).await.unwrap()
            } else {
                ctx.delete_application_emoji(EmojiId::new(emoji_data.emoji_id as u64)).await.expect("Failed to remove emoji");
                let emoji: Emoji = add_emoji(name, &img, &ctx).await;
                sqlx::query!("UPDATE emojis SET emoji_id = $2, checksum = $3 WHERE path = $1", path, emoji.id.get() as i64, hash_string)
                    .execute(pool)
                    .await
                    .expect("Wow something failed");
                emoji
            }
        },
        None => {
            let emoji: Emoji = add_emoji(name, &img, &ctx).await;
            sqlx::query!("INSERT INTO emojis (path, emoji_id, checksum) VALUES ($1, $2, $3)", path, emoji.id.get() as i64, hash_string)
                .execute(pool)
                .await
                .expect("Wow something failed");
            emoji
        }
    }
}

fn get_emoji_str(emoji: &Emoji) -> String {
    if !emoji.animated {
        format!("<:{}:{}>", emoji.name, emoji.id)
    } else {
        format!("<a:{}:{}>", emoji.name, emoji.id)
    }
}

pub async fn ensure_emojis(pool: &PgPool, ctx: &Context) -> EmojiHandler {
    let data = fs::read_to_string("data/emojis.json").unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();

    let mut emoji_handler = EmojiHandler::new();

    for (emoji_type, emoji_list) in json.as_object().unwrap() {
        let mut map = Map::new();
        for (obj_name, emoji_data) in emoji_list.as_object().unwrap() {
            let name = emoji_data["name"].as_str().unwrap();
            let path = emoji_data["path"].as_str().unwrap();

            let emoji = handle_emoji(&pool, &ctx, name, path).await;
            let emoji_string = get_emoji_str(&emoji);

            map.insert(obj_name.to_string(), Value::String(emoji_string.to_string()));

        }
        let obj = Value::Object(map);

        if emoji_type == "ui" {
            let ui_emojis: UiEmojis = serde_json::from_value(obj).expect("Incorrect Data");
            emoji_handler.set_ui_emojis(ui_emojis);
        }
    }
    emoji_handler
}
