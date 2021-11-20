use super::utils::shoppinglist::{set_shoppinglist,get_shopping_list};

pub async fn buy_handler(items: Vec<String>, chat_id: i64) -> Result<&'static str, &'static str> {
    if items.len() == 0 {
        return Err("Du hast nicht angegeben, was ich auf die Liste schreiben soll, ich ignoriere das mal 🙃")
    }

    // chain current list with new items and set shopping list
    set_shoppinglist(
        chat_id,
        get_shopping_list(chat_id)
            .unwrap().into_iter().chain(
                items.into_iter().map(|s| String::from(s.trim()))
            ).collect()
    )
    .and_then(|_| Ok("\\.\\.\\.ist notiert\\!"))
    .or_else(|_| Err("Fehler beim hinzufügen"))
}
