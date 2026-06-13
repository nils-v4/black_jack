use crate::card::*;
use rand::prelude::*;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn initialize(amount: u8) -> Self {
        let mut deck = Deck { cards: Vec::new() };
        for _ in 1..=amount {
            for suit in Suits::iter() {
                for num in [FaceCards::A, FaceCards::J, FaceCards::Q, FaceCards::K] {
                    let card = Card::new(num, &suit);
                    deck.add_card(card);
                }
                for num in (2..=10).map(|val| FaceCards::Value(val)) {
                    let card = Card::new(num, &suit);
                    deck.add_card(card);
                }
            }
        }
        return deck;
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn take_card(&mut self) -> Card {
        self.cards.pop().expect("no more cards left")
    }

    pub fn show_deck(&self) {
        for c in &self.cards {
            println!("{:?}", c);
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        let len = self.cards.len();
        for _c in 0..len {
            self.cards
                .swap(rng.random_range(0..len), rng.random_range(0..len));
        }
    }

    pub fn count(&self) -> usize {
        self.cards.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_1_deck() {
        let deck = Deck::initialize(1);
        assert_eq!(deck.count(), 52);
    }

    #[test]
    fn create_2_decks() {
        let deck = Deck::initialize(2);
        assert_eq!(deck.count(), 104);
    }

    #[test]
    fn test_take_card() {
        let mut deck = Deck::initialize(1);
        let initial_count = deck.count();
        let _card = deck.take_card();
        assert_eq!(deck.count(), initial_count - 1);
    }
}
