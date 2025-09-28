use chrono::Local;
use dotenvy::dotenv;
use log::{Level, LevelFilter, Metadata, Record, error, info};
use std::env;
use std::time::Duration;
use tokio::time::interval;
use twilight_http::Client as HttpClient;
use twilight_model::id::Id;
use twilight_model::id::marker::ChannelMarker;

struct BotLogger;

static LOGGER: BotLogger = BotLogger;

impl log::Log for BotLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            println!("[{}][{}] {}", timestamp, record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn init_logger() {
    log::set_logger(&LOGGER).expect("BotLogger should initialize only once");
    log::set_max_level(LevelFilter::Info);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    init_logger();

    let token = env::var("DISCORD_TOKEN")?;
    let channel_id_value = env::var("CHANNEL_ID")?.parse::<u64>()?;
    let channel_id: Id<ChannelMarker> = Id::new(channel_id_value);
    let interval_seconds = env::var("INTERVAL_SECONDS")?.parse::<u64>()?;

    let message = std::fs::read_to_string("message.txt")?;
    let message = message.trim_end_matches(&['\r', '\n'][..]).to_string();

    let client = HttpClient::new(token);
    let mut interval = interval(Duration::from_secs(interval_seconds));

    loop {
        interval.tick().await;

        let create_request = client.create_message(channel_id).content(&message);
        match create_request.await {
            Ok(_) => info!("Message sent successfully"),
            Err(why) => error!("Error sending message: {:?}", why),
        }
    }
}
