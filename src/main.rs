use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use serenity::async_trait;
use serenity::model::gateway::{Activity, Ready};
use serenity::prelude::*;
use gamedig::games::ohd;
struct Handler {
    is_loop_running: AtomicBool,
}
use simple_config_parser::Config;
#[async_trait]
impl EventHandler for Handler {


    // We use the cache_ready event just in case some cache operation is required in whatever use
    // case you have for this.
    async fn ready(&self, ctx: Context, ready:Ready) {
        println!("{} is connected!", ready.user.name);
        // it's safe to clone Context, but Arc is cheaper for this use case.
        // Untested claim, just theoretically. :P
        let ctx = Arc::new(ctx);

        // We need to check that the loop is not already running when this event triggers,
        // as this event triggers every time the bot enters or leaves a guild, along every time the
        // ready shard event triggers.
        //
        // An AtomicBool is used because it doesn't require a mutable reference to be changed, as
        // we don't have one due to self being an immutable reference.
        if !self.is_loop_running.load(Ordering::Relaxed) {
            // And of course, we can run more than one thread at different timings.
            let ctx = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    query(Arc::clone(&ctx)).await;
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });

            // Now that the loop is running, we set the bool to true
            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

async fn query(ctx: Arc<Context>) {
        let config2 = Config::new().file("config.cfg").unwrap();
        println!("Querying Game Data");
        let getip = config2.get_str("ip").unwrap();
        let getport = config2.get_str("port").unwrap();
        let ip = getip.as_str();
        let port : u16 = getport.parse().unwrap();
        let response = ohd::query(ip, Some(port));
        let info  = response.unwrap();
        let online = info.players_online;
        let servername = info.name;
        let available = info.players_maximum;
        println!("{}/{} {}", online, available, servername);
        let overall = format!("({}/{})", online, available);
        ctx.set_activity(Activity::watching(overall)).await;
}

#[tokio::main]
async fn main() {

    let config = Config::new().file("config.cfg").unwrap();
    let token = config.get_str("token").unwrap();
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}