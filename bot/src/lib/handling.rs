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
    if authenticated(chat_id).unwrap() {Ok(f)} else {Err("Du bist nicht authentifiziert")}
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

            cx.answer(match msg {Ok(f) => f().await.unwrap_or_else(|e| e), Err(e) => e})
        },
        Command::Authentifizieren(password) => {
            cx.answer(String::from(authenticate_handler(cx.chat_id(), password).await.unwrap_or_else(|e| {e})))
        },
        Command::Einkaufszettel => {
            let msg = only_if_auth(cx.chat_id(), || {
                show_handler(cx.chat_id())
            });

            cx.answer(match msg {Ok(f) => f().await.unwrap_or_else(|e| String::from(e)), Err(e) => String::from(e)})
        },
        Command::Eingekauft(item) => {
            match only_if_auth(cx.chat_id(), || { bought_handler(item, cx.chat_id()) }) {
                Ok(f) => { f().await
                    .and_then(|r|
                        match r {
                            Some(kbd) => Ok(cx.answer("Was hast du eingekauft?").reply_markup(kbd)),
                            None => Ok(cx.answer("Ist erledigt 🙂"))
                        }
                    )
                    .unwrap_or_else(|e| cx.answer(e))
                },
                Err(e) => cx.answer(e)
            }
        },
        Command::AllesEingekauft => {
            cx.answer(
                match only_if_auth(cx.chat_id(), || { all_bought_handler(cx.chat_id()) }) {
                    Ok(f) => f().await.unwrap_or_else(|e| e),
                    Err(e) => e
                }
            )
        }
    };

    tmsg.parse_mode(ParseMode::MarkdownV2).send().await?;

    Ok(())
}
