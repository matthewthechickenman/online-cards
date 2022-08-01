use super::{card_deck::{Card, Deck}, table::Table};

// 

pub struct MafiaGame {
    table: Table,
    mafia_count: u8,
}

pub fn generate_mafia_deck(players: u8, mafia: u8) -> Deck {
    let amount_black = players - mafia;
    let amount_red = mafia;

    let mut cards: Vec<Card> = Vec::new();

    cards.append(&mut vec![MAFIA_RED; amount_red.into()]);
    cards.append(&mut vec![MAFIA_BLACK; amount_black.into()]);

    Deck::new_as_hand(cards).shuffle()
}

const MAFIA_RED: Card = Card::get_red();
const MAFIA_BLACK: Card = Card::get_black();