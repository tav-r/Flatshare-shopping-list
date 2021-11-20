mod all_bought;
mod authenticate;
mod buy;
mod bought;
mod show_shoppinglist;
pub mod utils;

pub use buy::buy_handler;
pub use authenticate::authenticate_handler;
pub use show_shoppinglist::show_handler;
pub use bought::bought_handler;
pub use all_bought::all_bought_handler;


pub fn escape_special_chars(message: &str) -> String {
    let special_chars = "\\()_-~*[]'!";
    let mut new_str = String::from(message);

    for c in special_chars.chars() {
        new_str = new_str.replace(c, &format!("\\{}", c));
    }

    new_str
}
