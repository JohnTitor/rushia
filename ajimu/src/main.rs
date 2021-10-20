mod skills;

use std::env;

use egg_mode::KeyPair;
use rand::prelude::SliceRandom;
use skills::SKILLS;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let consumer_key = env::var("CONSUMER_KEY").expect("CONSUMER_KEY must be set");
    let consumer_secret = env::var("CONSUMER_SECRET").expect("CONSUMER_SECRET must be set");
    let access_key = env::var("ACCESS_KEY").expect("ACCESS_KEY must be set");
    let access_secret = env::var("ACCESS_SECRET").expect("ACCESS_SECRET must be set");

    let consumer = KeyPair::new(consumer_key, consumer_secret);
    let access = KeyPair::new(access_key, access_secret);
    let token = egg_mode::Token::Access { consumer, access };

    let name = SKILLS
        .choose(&mut rand::thread_rng())
        .expect("Failed to choose a skill; should NOT happen");

    let user_profile = egg_mode::account::UserProfile {
        name: Some(name.to_string()),
        ..Default::default()
    };

    if let Err(e) = egg_mode::account::update_profile(user_profile, &token).await {
        tracing::error!("Failed to update: {}", e);
    } else {
        tracing::info!("Succeeded to update!");
    }
}
