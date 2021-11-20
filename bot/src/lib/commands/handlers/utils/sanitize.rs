pub fn special_char(c: &char) -> bool {
    "_*[]()~`>#+-=|{}.!\\".chars().any(|d| d == *c)
}
 
pub fn encode(s: &str) -> String {
    s.chars().map(|c| if !special_char(&c) {
        String::from(c)
    } else {
        format!("\\{}", &c)
    }).collect::<Vec<String>>().join("")
}