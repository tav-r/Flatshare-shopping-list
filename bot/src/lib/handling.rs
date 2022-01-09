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
    escape_special_chars,
    Answer
};


// return the function if chat_id is authenticated, otherwise return wrapped error string
fn only_if_auth(f: Box<dyn Send + Fn(i64) -> Answer>) -> Box<dyn Send + Fn(i64) -> Answer> {
    Box::new(move |chat_id|
        if authenticated(chat_id).unwrap() {
            f(chat_id)
        } else {
            Answer::StaticText("Du bist nicht authentifiziert")
        }
    )
}

pub async fn handle_commands(
    cx: UpdateWithCx<Bot, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let process = match command {
        Command::Hilfe => Box::new(|_: i64| Answer::DynText(escape_special_chars(&Command::descriptions()))),
        Command::Einkaufen(items) => {
            only_if_auth(buy_handler(
                items.split(",").map(|s| String::from(s.trim())).filter(|s| s.len() > 0)  // split for comma, trim parts, filter empty strings
                    .collect()
            ))
        },
        Command::Authentifizieren(password) => {
            authenticate_handler(password)
        },
        Command::Einkaufszettel => {
            only_if_auth(show_handler())
        },
        Command::Eingekauft(item) => {
            only_if_auth(bought_handler(item))
        },
        Command::AllesEingekauft => {
            only_if_auth(all_bought_handler())
        }
    };

    // all sideffects happen below
    let tmsg = match process(cx.chat_id()) {
        Answer::StaticText(t) => cx.answer(t),
        Answer::DynText(s) => cx.answer(s),
        Answer::TextAndKeyboard(t, k) => cx.answer(t).reply_markup(k)
    };

    tmsg.parse_mode(ParseMode::MarkdownV2).send().await?;

    Ok(())
}
