use super::{card_deck::Deck};

#[derive(PartialEq)]
pub struct SeatedPlayer {
    cards: Deck,
    player: String,
    chips: u64,
    table: String,
}

#[derive(PartialEq)]
pub struct Player {
    pub id: String,
    nickname: String,
    socket: String,
}

impl SeatedPlayer {
    pub fn new(player: &Player, chips: u32, table: String) -> Self {
        SeatedPlayer {
            cards: Deck::BLANK,
            player: player.id.clone(),
            chips: chips as u64,
            table
        }
    }

    pub fn id(&self) -> String {
        self.player.clone()
    }
}