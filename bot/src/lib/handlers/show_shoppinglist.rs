use super::utils::{shoppinglist::get_shopping_list,sanitize::encode};
use super::Answer;


pub fn show_handler() -> Box<dyn Send + Fn(i64) -> Answer> {
    Box::new(move |chat_id: i64| {
        if let Ok(list) = get_shopping_list(chat_id) {
            let ans = match list.len() {
                0 => String::from("_\\(leer\\)_"),
                _ => list.into_iter()
                    .map(|s| encode(&s))
                    .map(|s: String| format!("✔️ {}", s))
                    .collect::<Vec<String>>().join("\n")
            };

            Answer::DynText(format!("🛍📋\n\n{}", ans))
        } else {
            Answer::StaticText("Ich konnte die Liste nicht abrufen\\.\\.\\. Bist Du authentifiziert?")
        }
    })
}
