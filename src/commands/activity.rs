use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, gateway::Activity},
    prelude::*,
};

#[command]
#[aliases("활동")]
#[description = "Sets the playing status"]
pub async fn activity(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let name = args.message();
    ctx.set_activity(Activity::playing(&name)).await;

    Ok(())
}
