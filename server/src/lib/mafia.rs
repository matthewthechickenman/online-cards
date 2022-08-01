use super::{card_deck::{Card, Deck}, table::{Table, ITable}, player::{Player}};

pub struct MafiaGame {
    mafia_count: u8,
    table: Table
}

impl MafiaGame {
    pub fn new(max_players: u8, host: Player, mafia_count: u8) -> Self {
        MafiaGame {
            mafia_count,
            table: Table::new(max_players, host, 0)
        }
    }
    pub fn generate_mafia_deck(&self) -> Deck {
        let amount_black = self.table.max_players - self.mafia_count;
        let amount_red = self.mafia_count;
    
        let mut cards: Vec<Card> = Vec::new();
    
        cards.append(&mut vec![MAFIA_RED; amount_red.into()]);
        cards.append(&mut vec![MAFIA_BLACK; amount_black.into()]);
    
        Deck::new_as_hand(cards).shuffle()
    }
}

impl ITable for MafiaGame {    
    fn add_spectator(&mut self, player: Player) -> &Self {
        self.table.spectate(player);
        self
    }

    fn seat_player(&mut self, player: Player) -> &Self {
        match self.table.seat(player) {
            Ok(_) => {},
            Err(_) => {
                // send player err (when WS implemented)
            } 
        };
        self
    }
}

const MAFIA_RED: Card = Card::get_red();
const MAFIA_BLACK: Card = Card::get_black();