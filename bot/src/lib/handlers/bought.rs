use super::utils::shoppinglist::{get_shopping_list,set_shoppinglist};
use super::Answer;
use teloxide::types::{KeyboardMarkup,KeyboardButton};


fn choose_from_keyboard(chat_id: i64) -> Answer {
    match get_shopping_list(chat_id) {
        Ok(l) => {
            if l.len() == 0 {
                Answer::StaticText("Es steht nichts auf dem Einkaufszettel 🤷🏾")
            } else {
                Answer::TextAndKeyboard(
                    "Was hast du eingekauft?",
                    KeyboardMarkup::new::<Vec<Vec<KeyboardButton>>>(
                        l.iter().map(|s|
                            vec!(KeyboardButton::new(format!("/eingekauft {}", s)))
                        ).collect()
                    )
                )
            }
        },
        Err(m) => Answer::StaticText(m)
    }
}


fn delete_from_list(chat_id: i64, item: &str) -> Answer {
     match get_shopping_list(chat_id) {
        Ok(l) => {
            if !l.iter().any(|i| i.eq(item.trim())) {
                Answer::StaticText("Das steht nicht auf dem Einkaufszettel 🤔")
            } else {
                Answer::StaticText(
                    match set_shoppinglist(
                        chat_id, l.into_iter().filter(|i| !&item.eq(i)).collect::<Vec<String>>()
                    ) {
                        Ok(()) => "Ist erledigt 🙂",
                        Err(msg) => msg
                    }
                )
            }
        },
        Err(m) => Answer::StaticText(m)
    }
}


pub fn bought_handler(item: String) -> Box<dyn Send + Fn(i64) -> Answer> {
    Box::new(move |chat_id: i64| {
        match item.len() {
            0 => choose_from_keyboard(chat_id),
            _ => delete_from_list(chat_id, &item)
        }
    })
}
