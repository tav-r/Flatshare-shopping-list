mod handlers;
pub mod handling;


use teloxide::utils::command::BotCommand;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Folgendes kann ich:")]
pub enum Command {
    #[command(description = "Zeigt diesen Text an")]
    Hilfe,
    #[command(description = "Etwas auf den Einkaufszettel schreiben")]
    Einkaufen(String),
    #[command(description = "Mit Passwort authentifizieren")]
    Authentifizieren(String),
    #[command(description = "Einkaufszettel anzeigen")]
    Einkaufszettel,
    #[command(description = "Etwas von der Einkaufsliste streichen")]
    Eingekauft(String),
    #[command(description = "Den alten Einkaufszettel wegwerfen")]
    AllesEingekauft,
}

