use super::utils::shoppinglist::{set_shoppinglist,get_shopping_list};
use super::Answer;


pub fn buy_handler(items: Vec<String>) -> Box<dyn Send + Fn(i64) -> Answer> {
    Box::new(move |chat_id: i64| {
        if items.len() == 0 {
            return Answer::StaticText("Du hast nicht angegeben, was ich auf die Liste schreiben soll, ich ignoriere das mal 🙃")
        }

        // chain current list with new items and set shopping list
        Answer::StaticText(
            set_shoppinglist(
                chat_id,
                get_shopping_list(chat_id)
                    .unwrap().into_iter().chain(
                        items.clone().into_iter().map(|s| String::from(s.trim()))
                    ).collect()
            )
            .and_then(|_| Ok("\\.\\.\\.ist notiert\\!"))
            .unwrap_or("Fehler beim hinzufügen")
        )
    })
}
