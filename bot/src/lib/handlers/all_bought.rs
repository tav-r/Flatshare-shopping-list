use super::utils::shoppinglist::delete_shoppinglist;


pub async fn all_bought_handler(chat_id: i64) -> Result<&'static str, &'static str> {
    delete_shoppinglist(chat_id)
        .and_then(|_| Ok("Einkaufszettel wurde gelöscht\\!"))
        .or_else(|e| Err(e))
}
