mod lib;

extern crate redis;

use teloxide::prelude::*;
use lib::commands::{handling::handle_commands,Command};
use std::env;

async fn run() {
    teloxide::enable_logging!();

    let bot = Bot::from_env();

    teloxide::commands_repl(bot, env::var("BOT_NAME").unwrap(), |cx: UpdateWithCx<Bot, Message>, command: Command| async {
        handle_commands(cx, command).await
    }).await;
}

#[tokio::main]
async fn main() {
    run().await;
}
