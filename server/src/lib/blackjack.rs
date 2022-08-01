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

    pub fn deal_hand(&mut self) {
        self.dealer_hand = self.table.deck.take_from_top(2);
        for player in &mut self.table.players {
            player.set_hand(self.table.deck.take_from_top(2))
        }
    }

    pub fn play_game(&mut self) -> &Self {
        // Get all user input and act upon it.
        let dealer_val = calculate_blackjack_value(&(&self).dealer_hand);
        if dealer_val < 17 {
            // Deal new card to dealer
        }
        return self;
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

fn calculate_blackjack_value(deck: &Deck) -> u16 {
    let mut val: u16 = 0;
    let mut aces: u8 = 0;
    for card in deck.inner() {
        match card.get_value() {
            10 | 11 | 12 | 13 => { val += 10 as u16; },
            1 => {
                aces += 1;
                if val > 11 {
                    val += 1;
                } else {
                    val += 11
                }
            }
            _ => { val += card.get_value() as u16 }
        }
    }
    while val > 21 && aces > 1 {
        aces = aces - 1;
        val = val - 10;
    };

    return val;
}