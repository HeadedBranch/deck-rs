use std::fmt::Display;
use crate::CardValue::*;
use crate::Suit::*;
use rand::rng;
use rand::seq::SliceRandom;

pub struct Deck {
    deck: Vec<Card>,
    shuffled: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Card {
    value: CardValue,
    suit: Suit,
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug, Hash)]
pub enum CardValue {
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl From<char> for Suit {
    fn from(char: char) -> Suit {
        match char {
            'H' => Hearts,
            'D' => Diamonds,
            'C' => Clubs,
            'S' => Spades,
            _ => Spades,
        }
    }
}

impl From<char> for CardValue {
    fn from(char: char) -> CardValue {
        match char {
            'A' => Ace,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            _ => Ace,
        }
    }
}
impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.deck.iter() {
            write!(f, "{}", i)?;
        }
        Ok(())
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

impl Display for CardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Ace => "A",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "10",
            Jack => "J",
            Queen => "Q",
            King => "K",
        })
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Hearts => "H",
            Diamonds => "D",
            Clubs => "C",
            Spades => "S",
        })
    }
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Vec::new();
        const SUITS: [Suit; 4] = [Spades, Diamonds, Clubs, Hearts];
        const VALUES: [CardValue; 13] = [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ];
        for a in 0..4 {
            for b in 0..13 {
                deck.push(Card {
                    value: if a <= 2 { VALUES[b] } else { VALUES[12 - b] },
                    suit: SUITS[a],
                })
            }
        }
        Deck {
            deck,
            shuffled: false,
        }
    }
    pub fn new_shuffled() -> Deck {
        let mut deck = Deck::new();
        let mut rng = rng();
        deck.deck.shuffle(&mut rng);
        deck.shuffled = true;
        deck
    }
    pub fn new_custom (deck: Vec<Card>) -> Deck {
        Deck { deck, shuffled: false }
    }
    pub fn from_str(input: &str) -> Option<Deck> {
        let mut deck = Vec::new();
        let mut input = input.bytes();
        loop {
            let rank: char = input.next()?.into();
            let suit: char = input.next()?.into();
            deck.push(Card::new(rank.into(), suit.into()));
            if input.len() == 0 {
                break Some(Deck { deck, shuffled: true })
            }
        }
    }
    pub fn shuffle(&mut self) {
        self.shuffled = true;
        let mut rng = rng();
        self.deck.shuffle(&mut rng);
    }
    pub fn deck(&self) -> &Vec<Card> {
        &self.deck
    }
    pub fn shuffled(&self) -> bool {
        self.shuffled
    }
    pub fn size(&self) -> usize {
        self.deck.len()
    }
}

impl Card {
    pub fn new(value: CardValue, suit: Suit) -> Card {
        Card { value, suit }
    }
    pub fn value(&self) -> CardValue {
        self.value
    }
    pub fn suit(&self) -> Suit {
        self.suit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shuffled_values() {
        let mut deck = Deck::new();
        let shuffled_deck = Deck::new_shuffled();
        assert_eq!(deck.shuffled, false);
        assert_eq!(shuffled_deck.shuffled, true);
        deck.shuffle();
        assert_eq!(deck.shuffled, true);
    }
    #[test]
    fn deck_size() {
        let deck = Deck::new();
        assert_eq!(deck.deck.len(), 52)
    }
    #[test]
    fn card_value_printing() {
        assert_eq!(format!("{}", Card::new(Ace, Spades)), "AS");
        assert_eq!(format!("{}", Card::new(Ace, Hearts)), "AH");
        assert_eq!(format!("{}", Card::new(Two, Spades)), "2S");
    }
    #[test]
    fn deck_from_string() {
        let deck = Deck::from_str("AS2C5D").unwrap();
        assert_eq!(deck.deck[0], Card::new(Ace, Spades));
        assert_eq!(deck.deck[1], Card::new(Two, Clubs));
        assert_eq!(deck.deck[2], Card::new(Five, Diamonds));
    }
}
