#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod settings;
mod events;
mod commands;

use std::sync::RwLock;

use serenity::{
    client::Client,
    framework::{
        StandardFramework,
        standard::macros::group
    }
};

use settings::Settings;
use events::Handler;
use commands::{
    ping::*
};

lazy_static! {
    static ref SETTINGS : RwLock<Settings> = RwLock::new(Settings::new().unwrap());
}

group!({
    name: "general",
    options: {},
    commands: [ping],
});

fn main() {

    simple_logger::init_with_level(log::Level::Info).unwrap();

    let settings = &*SETTINGS.read().unwrap();
    let mut client = Client::new(&settings.discord.token, Handler).expect("Error creating the client!");

    client.with_framework(StandardFramework::new()
    .configure(|c| c
        .prefix("R~"))
    .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        panic!("An error occurred while running the client: {:?}", why);
    }
}
