use crate::lib::{table::Table, card_deck::Deck};

use super::{table::ITable, player::Player};

pub struct BlackjackTable {
    table: Table,
    dealer_hand: Deck,
}

impl BlackjackTable {
    fn new(max_players: u8, host: Player, starting_chips: u32) -> Self {
        let deck = Deck::new(2).shuffle();
        BlackjackTable {
            table: Table::new(max_players, host, starting_chips, deck),
            dealer_hand: Deck::BLANK
        }
    }

    fn deal_hand(&mut self) {
        self.dealer_hand = self.table.deck.take_from_top(2);
        for player in &mut self.table.players {
            player.set_hand(self.table.deck.take_from_top(2))
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