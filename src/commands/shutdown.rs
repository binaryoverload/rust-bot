use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use log::error;

use crate::ShardManagerContainer;

#[command]
pub fn shutdown(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let shard_manager = match data.get::<ShardManagerContainer>() {
       Some(v) => v,
       None => {
           let _ = msg.reply(&ctx, "There was a problem getting the shard manager");
           error!("There was a problem getting the shard manager, oops?");
           return Ok(());
       },
   };

   let mut manager = shard_manager.lock();
   manager.shutdown_all();
    Ok(())
}
