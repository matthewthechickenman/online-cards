use super::{card_deck::{Card, Deck}, table::{Table, ITable}, player::{Player}};

pub struct MafiaGame {
    table: Table
}

impl MafiaGame {
    pub fn new(max_players: u8, host: Player, mafia_count: u8) -> Self {
        MafiaGame {
            table: Table::new(max_players, host, 0, generate_mafia_deck(max_players, mafia_count))
        }
    }

    pub fn deal_hand(&mut self) -> &mut Self {
        for player in &mut self.table.players {
            player.set_hand(self.table.deck.take_from_top(1))
        }
        self
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

pub fn generate_mafia_deck(max_players: u8, mafia_count: u8) -> Deck {
    let amount_black = max_players - mafia_count;
    let amount_red = mafia_count;

    let mut cards: Vec<Card> = Vec::new();

    cards.append(&mut vec![MAFIA_RED; amount_red.into()]);
    cards.append(&mut vec![MAFIA_BLACK; amount_black.into()]);

    Deck::new_as_hand(cards).shuffle()
}