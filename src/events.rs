use serenity::prelude::EventHandler;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use log::info;

pub struct Handler;

impl EventHandler for Handler {

    fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.tag());
    }

}
