#[macro_use]
extern crate lazy_static;

pub mod lib;

use std::collections::HashMap;

use lib::card_deck;
use lib::mafia::generate_mafia_deck;
use lib::player::Player;
use lib::table::Games;

lazy_static!{
    static ref PLAYERS: Vec<Player> = Vec::new();
    static ref GAMES: HashMap<String, Games> = HashMap::new();
}

fn main() {
    let mut deck = card_deck::Deck::new(1);
    deck = deck.shuffle();
    println!("List of shuffled cards: {}", &deck.clone().print());
    println!("Amount of cards: {}", &deck.length());

    println!("Mafia Deck: {}", generate_mafia_deck(10, 2).print())
}
