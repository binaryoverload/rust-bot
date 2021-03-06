use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::utils::Colour;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use log::error;

use crate::ShardManagerContainer;

#[command]
pub fn shutdown(ctx: &mut Context, msg: &Message, mut _args: Args) -> CommandResult {
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
   msg.channel_id.send_message(&ctx.http, |m| {
       m.embed(|e| {
           e.colour(Colour::RED).description("Shutting down, goodbye!")
       });
       m
   })?;
   manager.shutdown_all();
   Ok(())
}
