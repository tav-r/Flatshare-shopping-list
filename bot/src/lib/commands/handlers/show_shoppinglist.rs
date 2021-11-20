use super::utils::{shoppinglist::get_shopping_list,sanitize::encode};


pub async fn show_handler(chat_id: i64) -> Result<String, &'static str> {
    if let Ok(list) = get_shopping_list(chat_id) {
        let ans = match list.len() {
            0 => String::from("_\\(leer\\)_"),
            _ => list.into_iter()
                .map(|s| encode(&s))
                .map(|s: String| format!("✔️ {}", s))
                .collect::<Vec<String>>().join("\n")
        };

        Ok(format!("🛍📋\n\n{}", ans))
    } else {
        Err("Ich konnte die Liste nicht abrufen\\.\\.\\. Bist Du authentifiziert?")
    }
}
