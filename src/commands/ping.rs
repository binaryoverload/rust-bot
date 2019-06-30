use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub fn ping(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong")?;
    Ok(())
}
