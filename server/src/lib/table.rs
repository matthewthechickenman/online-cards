use super::{
    player::{Player, SeatedPlayer}, 
    generate_new_id, 
    blackjack::BlackjackTable, 
    mafia::MafiaGame, card_deck::Deck
};

pub enum Games {
    Blackjack(BlackjackTable),
    Mafia(MafiaGame)
}
pub trait ITable {
    fn add_spectator(&mut self, player: Player) -> &Self;
    fn seat_player(&mut self, player: Player) -> &Self;
}


pub struct Table {
    pub players: Vec<SeatedPlayer>,
    pub spectators: Vec<Player>,
    pub max_players: u8,
    host: Player,
    pub id: String,
    starting_chips: u32,
    pub deck: Deck,
}

impl Table {
    pub fn new(max_players: u8, host: Player, starting_chips: u32, deck: Deck) -> Self {
        Table {
            max_players,
            players: Vec::new(),
            spectators: Vec::new(),
            host,
            id: generate_new_id(),
            starting_chips,
            deck
        }
    }

    pub fn spectate(&mut self, player: Player) -> &Self {
        self.spectators.push(player);
        self
    }

    pub fn seat(&mut self, player: Player) -> Result<&Self, &str> {
        if self.players.len() < self.max_players.into() {
            self.spectators.retain(|x| x != &player);
            self.players.push(
                SeatedPlayer::new(
                    &player,
                    self.starting_chips as u32,
                    self.id.clone()
                )
            );
            Ok(self)
        } else {
            Err("Could not seat - too many players at table.")
        }

        
    }

    pub fn kick(&mut self, to_kick: Player, request_by: Player) -> &Self {
        if request_by != self.host {
            self.players.retain(|x| x.id() != to_kick.id);

        }
        self
    }
}

