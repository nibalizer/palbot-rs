use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use reqwest;

struct Handler;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIResponse {
    pub content: Vec<Content>,
    pub page: i64,
    pub limit: i64,
    pub count: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub id: i64,
    pub key: String,
    pub image: String,
    pub name: String,
    pub wiki: String,
    pub types: Vec<String>,
    pub image_wiki: String,
    pub suitability: Vec<Suitability>,
    pub drops: Vec<String>,
    pub aura: Aura,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Suitability {
    #[serde(rename = "type")]
    pub type_field: String,
    pub level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aura {
    pub name: String,
    pub description: String,
}


fn format_output(parsed: APIResponse, output: &mut String) {

    let pal = &parsed.content[0].name;
    println!("Pal name: {pal}");

    let types: &String = &parsed.content[0].types.join(", ");
    let suits: &String = &parsed.content[0].suitability.iter()
        .map(|s| s.type_field.clone() + ",")
        .collect();

    *output = format!("Pal: {}, Types: {}, Suitabilities: {}",pal, types, suits);

}


#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message is received - the
    // closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be dispatched
    // simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content.starts_with("!pal") {
            // Pal lookup

            let pal = msg.content.split(" ").nth(1).unwrap();

            println!("Pal name: {pal}");
            // The URL of the paldb api + pal name
            let url = format!("http://localhost:8080/?name={}", pal);
            println!("url: {url}");

            // Send a GET request to the specified URL
            let response = reqwest::get(url).await.unwrap();
            let mut output = String::from("temp");

            match response.status() {
                reqwest::StatusCode::OK => {
                    // on success, parse our JSON to an APIResponse
                    match response.json::<APIResponse>().await {
                        Ok(parsed) => format_output(parsed, &mut output),
                        Err(_) => println!("Hm, the response didn't match the shape we expected."),
                    };
                }
                reqwest::StatusCode::UNAUTHORIZED => {
                    println!("Need to grab a new token");
                }
                other => {
                    panic!("Uh oh! Something unexpected happened: {:?}", other);
                }
            };


            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, output).await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
