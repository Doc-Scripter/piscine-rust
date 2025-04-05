use rand::prelude::*;
use rand::rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn random() -> Suit {
        let suits: [Suit; 4] = [Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club];
        let mut rng: ThreadRng = rng();
        *suits.choose(&mut rng).unwrap()
    }

    pub fn translate(value: u8) -> Suit {
        match value as i32 {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let ranks = [
            Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
            Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King,
        ];
        let mut rng = rng();
        *ranks.choose(&mut rng).unwrap()
    }

    pub fn translate(value: u8) -> Rank {
        match value as i32 {
            1 => Rank::Ace,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            _ =>Rank::King,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    // Ace of Spades is a winner
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
