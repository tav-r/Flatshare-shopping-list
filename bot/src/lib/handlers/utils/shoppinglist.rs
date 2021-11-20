use std::env;
use redis::Commands;

pub fn get_shopping_list(chat_id: i64) -> Result<Vec<String>, &'static str> {
    redis::Client::open(env::var("REDIS_URL").unwrap())
        .and_then(|mut conn| conn.lrange(&format!("{}", chat_id), 0, -1))
        .and_then(|v| Ok(v))
        .or_else(|_| Err("Error opening database connection when retrieving shopping list"))
}

pub fn set_shoppinglist(chat_id: i64, items: Vec<String>) -> Result<(), &'static str> {
    redis::Client::open(env::var("REDIS_URL").unwrap())  // open connection
        .and_then(|mut conn| {
            conn.del::<i64, ()>(chat_id).unwrap(); Ok(conn)  // delete list for chat_id
        })
        .and_then(|mut conn| {
            items.iter().for_each(|i| conn.rpush(&format!("{}", chat_id), i).unwrap()); Ok(())  // set new shopping list
        })
        .or_else(|_| Err("Error opening database connection when setting shopping list"))
}

pub fn delete_shoppinglist(chat_id: i64) -> Result<(), &'static str> {
    redis::Client::open(env::var("REDIS_URL").unwrap())
    .and_then(|mut conn| {conn.del::<i64, ()>(chat_id).unwrap(); Ok(())})
    .or_else(|_| Err("Error opening database connection when deleting shopping list"))
}
