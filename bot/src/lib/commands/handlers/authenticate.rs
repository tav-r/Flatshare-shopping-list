use super::utils::auth::{authenticate};

pub async fn authenticate_handler(chat_id: i64, password: String) -> Result<&'static str, &'static str> {
    authenticate(chat_id, &password)
        .and_then(|r| Ok(r))
        .or_else(|_| Err("Error while authenticating"))
}
