use super::utils::shoppinglist::{get_shopping_list,set_shoppinglist};
use teloxide::types::{KeyboardMarkup,KeyboardButton};

// Returns Ok if all worked well and Ok wraps a Result<KeyboardMarkup> if  no item was specified, otherwise it wraps None
pub async fn bought_handler(item: String, chat_id: i64) -> Result<Option<KeyboardMarkup>, &'static str> {
    if item.len() == 0 {
        get_shopping_list(chat_id)
            .and_then(|l| Ok(Some(KeyboardMarkup::new::<Vec<Vec<KeyboardButton>>>(
                l.iter().map(|s| 
                    vec!(KeyboardButton::new(format!("/eingekauft {}", s)))
                ).collect()
            ))))
            .or_else(|_| Err("Konnte die Liste nicht abrufen 😧"))
    } else {
        set_shoppinglist(
            chat_id,
            get_shopping_list(chat_id)
                .and_then(
                    |l| Ok(l.into_iter().filter(
                        |i| !&item.eq(i)
                    ).collect::<Vec<String>>())
                ).unwrap()
        ).unwrap();

        Ok(None)
    }
}
