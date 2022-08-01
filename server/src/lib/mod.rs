pub mod card_deck;
pub mod blackjack;
pub mod player;
pub mod table;
pub mod roulette;
pub mod mafia;

pub fn generate_new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}