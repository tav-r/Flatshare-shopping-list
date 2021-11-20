use std::{env,error::Error};
use redis::Commands;
use crypto_hash::{Algorithm,hex_digest};

pub fn authenticated(chat_id: i64) -> Result<bool,Box<dyn Error>> {
    let ids: Vec<i64> = redis::Client::open(env::var("REDIS_URL")?)?
        .lrange("auth", 0, -1)?;

    Ok(ids.iter().any(|id| *id == chat_id))
}

pub fn authenticate(chat_id: i64, password: &str) -> Result<&'static str, Box<dyn Error>> {
    Ok(redis::Client::open(env::var("REDIS_URL")?)
        .and_then(
            |mut conn| {
                if hex_digest(Algorithm::SHA512, password.as_bytes()) == env::var("BOT_PASSWORD_HASH").unwrap() {
                    conn.rpush("auth", chat_id)?;
                    Ok("Authentisiert\\!")
                } else {
                    Ok("Sorry, falsches Passwort\\.\\.\\.")
                }
            }
        )?
    )
}
