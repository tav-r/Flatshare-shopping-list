use super::utils::shoppinglist::{get_shopping_list,set_shoppinglist};
use teloxide::types::{KeyboardMarkup,KeyboardButton};

// Returns Ok if all worked well and Ok wraps a Result<KeyboardMarkup> if  no item was specified, otherwise it wraps None
pub async fn bought_handler(item: String, chat_id: i64) -> Result<Option<KeyboardMarkup>, &'static str> {
    if item.len() == 0 {
        get_shopping_list(chat_id)
            .and_then(|l| if l.len() == 0 {
                Err("Es steht nichts auf dem Einkaufszettel 🤷🏾")
            } else {
                Ok(l)
            })
            .and_then(|l| Ok(Some(KeyboardMarkup::new::<Vec<Vec<KeyboardButton>>>(
                l.iter().map(|s| 
                    vec!(KeyboardButton::new(format!("/eingekauft {}", s)))
                ).collect()
            ))))
            .or_else(|e| Err(e))
    } else {
        get_shopping_list(chat_id)
            .and_then(|l| if !l.iter().any(|i| i.eq(item.trim())) {
                Err("Das steht nicht auf dem Einkaufszettel 🤔")
            } else {
                Ok(l)
            })
            .and_then(
                |l| Ok(l.into_iter().filter(
                    |i| !&item.eq(i)
                ).collect::<Vec<String>>())
            )
            .and_then(
                |v| { set_shoppinglist(chat_id, v).and_then(|_| Ok(None)).or_else(|e| Err(e)) }
            )
            .or_else(|e| Err(e))
    }
}
