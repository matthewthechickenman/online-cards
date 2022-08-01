use rand::{seq::SliceRandom, thread_rng};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new(deck_amount: u8) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for _ in 1..=deck_amount {
            cards.append(&mut generate_deck());
        }
        Deck(cards).shuffle()
    }

    pub fn new_as_hand(cards: Vec<Card>) -> Self {
        Deck(cards)
    }

    pub fn inner(&self) -> &Vec<Card> {
        &self.0
    }

    pub const BLANK: Deck = Deck(vec![]);

    pub fn take_from_top(&mut self, card_amount: u8) -> Deck {
        let mut vec: Vec<Card> = Vec::new();
        for _ in 1..=card_amount {
            match self.0.pop() {
                Some(card) => {
                    vec.push(card);
                }
                None => {}
            }
        }
        Deck(vec)
    }

    pub fn shuffle(mut self) -> Self {
        self.0.shuffle(&mut thread_rng());
        self
    }

    pub fn print(&self) -> String {
        let mut str = String::new();
        for card in &self.0 {
            str.push_str(format!("{}, ", card).as_str())
        }
        str.pop();
        str.pop();
        str
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }
}

fn generate_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for s in Suit::VALUES {
        for i in 1..=13 {
            cards.push(Card::new(s.clone(), i));
        }
    }
    return cards;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    value: u8, // 1 (A) - 13 (K). 11 J, 12 Q, 13 K, 1 A
    suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.value {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            _ => self.value.to_string(),
        };
        write!(f, "{}{}", value, self.suit)
    }
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Self {
        Card { suit, value }
    }

    pub const fn get_red() -> Self {
        Card {
            value: 1,
            suit: Suit::Diamond,
        }
    }

    pub const fn get_black() -> Self {
        Card {
            value: 1,
            suit: Suit::Clubs,
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Suit {
    Diamond,
    Hearts,
    Spades,
    Clubs,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self {
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
            Suit::Diamond => "♦",
            Suit::Hearts => "♥",
        };
        write!(f, "{}", suit)
    }
}

impl Suit {
    const VALUES: [Self; 4] = [Self::Diamond, Self::Hearts, Self::Spades, Self::Clubs];
}
