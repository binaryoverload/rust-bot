#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod settings;
mod events;
mod commands;

use std::sync::{Arc, RwLock};

use serenity::{
    client::{
        Client,
        bridge::gateway::ShardManager
    },
    framework::{
        StandardFramework,
        standard::macros::group
    },
    prelude::*
};

use settings::Settings;
use events::Handler;
use commands::{
    ping::*,
    shutdown::*
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

lazy_static! {
    static ref SETTINGS : RwLock<Settings> = RwLock::new(Settings::new().unwrap());
}

group!({
    name: "general",
    options: {},
    commands: [ping],
});

group!({
    name: "admin",
    options: {},
    commands: [shutdown],
});

fn main() {

    simple_logger::init_with_level(log::Level::Info).unwrap();

    let settings = &*SETTINGS.read().unwrap();
    let mut client = Client::new(&settings.discord.token, Handler).expect("Error creating the client!");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    client.with_framework(StandardFramework::new()
    .configure(|c| c
        .prefix("R~"))
    .group(&GENERAL_GROUP)
    .group(&ADMIN_GROUP));

    if let Err(why) = client.start() {
        panic!("An error occurred while running the client: {:?}", why);
    }
}
