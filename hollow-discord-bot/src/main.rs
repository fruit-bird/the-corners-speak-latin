use hollow::Hollow;
use serenity::{async_trait, model::channel::Message, prelude::*};
use std::{env, error::Error};

const BOT_PREFIX: &str = "/hollow";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        eprintln!("{}", msg.content);

        if msg.content == BOT_PREFIX {
            let hollow = Hollow::default();
            let content = hollow.run().await.unwrap_or("no hollow".to_string());

            msg.channel_id
                .say(&ctx, content)
                .await
                .expect("Couldn't send message");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let token = env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await?;

    client.start().await?;
    Ok(())
}
