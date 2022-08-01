use crate::lib::{table::Table, card_deck::Deck};

use super::{table::ITable, player::Player};

pub struct BlackjackDealer {
    hand: Deck
}

pub struct BlackjackTable {
    table: Table,
    dealer: BlackjackDealer
}

impl BlackjackTable {
    fn new(max_players: u8, host: Player, starting_chips: u32) -> Self {
        BlackjackTable {
            table: Table::new(max_players, host, starting_chips),
            dealer: BlackjackDealer { hand: Deck::BLANK }
        }
    }
}

impl ITable for BlackjackTable {
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