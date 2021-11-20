use teloxide::{prelude::*, utils::command::BotCommand,types::ParseMode};
use std::error::Error;
use super::Command;
use super::handlers::utils::auth::authenticated;
use super::handlers::{
    buy_handler,
    authenticate_handler,
    show_handler,
    bought_handler,
    all_bought_handler,
    escape_special_chars
};


pub async fn handle_commands(
    cx: UpdateWithCx<Bot, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let tmsg = match command {
        Command::Hilfe => cx.answer(escape_special_chars(&Command::descriptions())),
        Command::Einkaufen(items) => {
            let msg = if authenticated(cx.chat_id()).unwrap() {
                String::from(buy_handler(
                    items.split(",").map(|s| String::from(s.trim())).filter(|s| s.len() > 0)  // split for comma, trim parts
                        .collect(),
                    cx.chat_id()
                ).await.unwrap_or_else(|msg| msg))
            } else {
                String::from("Du kannst nichts auf den Zettel schreiben, du bist nicht authentifiziert\\.")
            };

            cx.answer(msg)
        },
        Command::Authentifizieren(password) => {
            let msg = String::from(authenticate_handler(cx.chat_id(), password).await.unwrap_or_else(|msg| {msg}));

            cx.answer(msg)
        },
        Command::Einkaufszettel => {
            let msg = if authenticated(cx.chat_id()).unwrap() {
                String::from(show_handler(cx.chat_id()).await.unwrap_or_else(|msg| {String::from(msg)}))
            } else {
                String::from("Du hast keinen Einkaufszettel, du bist nicht authentifiziert\\.")
            };

            cx.answer(msg)
        },
        Command::Eingekauft(item) => {
            if authenticated(cx.chat_id()).unwrap() {
                bought_handler(item, cx.chat_id()).await
                    .and_then(|r| match r {
                        Some(kbd) => Ok(cx.answer("Was hast du eingekauft?").reply_markup(kbd)),
                        None => Ok(cx.answer("Ist erledigt 🙂"))
                    })
                    .unwrap_or_else(|e| cx.answer(e))
            } else {
                cx.answer("Du kannst nichts vom Zettel streichen, du bist nicht authentifiziert\\.")
            }
        },
        Command::AllesEingekauft => {
            let msg = if authenticated(cx.chat_id()).unwrap() {
                String::from(all_bought_handler(cx.chat_id()).await.unwrap_or_else(|msg| {msg}))
            } else {
                String::from("Du kannst den Zettel nicht wegwerfen, du bist nicht authentifiziert\\.")
            };

            cx.answer(msg)
       }
    };

    tmsg.parse_mode(ParseMode::MarkdownV2).send().await?;

    Ok(())
}
