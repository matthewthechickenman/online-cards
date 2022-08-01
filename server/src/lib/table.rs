use super::{
    player::{Player, SeatedPlayer}, 
    generate_new_id, 
    blackjack::BlackjackTable, 
    mafia::MafiaGame
};

pub enum Games {
    Blackjack(BlackjackTable),
    Mafia(MafiaGame)
}
pub trait ITable {
    fn new(max_players: u8, host: Player, default_chips: u32) -> Self;

    fn add_spectator(&mut self, player: Player) -> &Self;
    fn seat_player(&mut self, player: Player) -> &Self;
}


pub struct Table {
    players: Vec<SeatedPlayer>,
    spectators: Vec<Player>,
    max_players: u8,
    host: Player,
    id: String,
    starting_chips: u32,
}

impl Table {
    pub fn new(max_players: u8, host: Player, starting_chips: u32) -> Self {
        Table {
            max_players,
            players: Vec::new(),
            spectators: Vec::new(),
            host,
            id: generate_new_id(),
            starting_chips
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
}

