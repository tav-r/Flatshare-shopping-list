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


// return the function if chat_id is authenticated, otherwise return wrapped error string
fn only_if_auth<Fn>(chat_id: i64, f: Fn) -> Result<Fn, &'static str> {
    if authenticated(chat_id).unwrap() {Ok(f)} else {Err("Du bist nicht authentisiert")}
}

// main "loop"
pub async fn handle_commands(
    cx: UpdateWithCx<Bot, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let tmsg = match command {
        Command::Hilfe => cx.answer(escape_special_chars(&Command::descriptions())),
        Command::Einkaufen(items) => {
            let msg = only_if_auth(cx.chat_id(), || { buy_handler(
                items.split(",").map(|s| String::from(s.trim())).filter(|s| s.len() > 0)  // split for comma, trim parts, filter empty strings
                    .collect(),
                cx.chat_id()
            )});

            cx.answer(match msg {Ok(f) => f().await.unwrap(), Err(m) => m})
        },
        Command::Authentifizieren(password) => {
            cx.answer(String::from(authenticate_handler(cx.chat_id(), password).await.unwrap_or_else(|msg| {msg})))
        },
        Command::Einkaufszettel => {
            let msg = only_if_auth(cx.chat_id(), || {
                show_handler(cx.chat_id())
            });

            cx.answer(match msg {Ok(f) => f().await.unwrap(), Err(m) => String::from(m)})
        },
        Command::Eingekauft(item) => {
            match only_if_auth(cx.chat_id(), || { bought_handler(item, cx.chat_id()) }) {
                Ok(f) => {
                    match f().await.unwrap() {
                        Some(kbd) => cx.answer("Was hast du eingekauft?").reply_markup(kbd),
                        None => cx.answer("Ist erledigt 🙂")
                    }
                },
                Err(m) => cx.answer(m)
            }
        },
        Command::AllesEingekauft => {
            cx.answer(
                match only_if_auth(cx.chat_id(), || { all_bought_handler(cx.chat_id()) }) {
                    Ok(f) => f().await.unwrap(),
                    Err(m) => m
                }
            )
       }
    };

    tmsg.parse_mode(ParseMode::MarkdownV2).send().await?;

    Ok(())
}
