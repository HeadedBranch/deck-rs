use crate::CardValue::*;
use crate::Suit::*;
use rand::rng;
use rand::seq::SliceRandom;

pub struct Deck {
    deck: Vec<Card>,
    shuffled: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    value: CardValue,
    suit: Suit,
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
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
    fn deck_order() {
        let deck = Deck::new();
        assert_eq!(
            deck.deck[0],
            Card {
                value: Ace,
                suit: Spades
            }
        );
    }
}
