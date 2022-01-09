use super::utils::shoppinglist::delete_shoppinglist;
use super::Answer;


pub fn all_bought_handler() -> Box<dyn Send + Fn(i64) -> Answer> {
    Box::new(
        move |chat_id: i64| Answer::StaticText(
            match delete_shoppinglist(chat_id) {
                Ok(_) => "Einkaufszettel wurde gelöscht\\!",
                Err(m) => m
            }
        )
    )
}
