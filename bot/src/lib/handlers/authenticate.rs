use super::utils::auth::{authenticate};
use super::Answer;


pub fn authenticate_handler(password: String) -> Box<dyn Send + Fn(i64) -> Answer> {
    Box::new(
        move |chat_id: i64| Answer::StaticText(authenticate(chat_id, &password)
            .unwrap_or("Error while authenticating"))
    )
}
