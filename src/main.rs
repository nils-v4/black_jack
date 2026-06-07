use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone)]
enum FaceCards {
    Value(u8),
    A,
    J,
    Q,
    K,
}

#[derive(EnumIter, Debug, Clone)]
enum Suits {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Debug)]
struct Card {
    number: FaceCards,
    suit: Suits,
}

impl Card {
    fn new(num: FaceCards, suit: &Suits) -> Self {
        Card {
            number: num.clone(),
            suit: suit.clone(),
        }
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn initialize(amount: u8) -> Self {
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

    fn show_deck(&self) {
        for c in &self.cards {
            println!("{:?}", c);
        }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::rng();
        let len = self.cards.len();
        for _c in 0..len {
            self.cards
                .swap(rng.random_range(0..len), rng.random_range(0..len));
        }
    }

    fn count(&self) {
        println!("{:?}", self.cards.len());
    }
}

fn main() {
    let mut deck = Deck::initialize(1);
    deck.shuffle();
    deck.show_deck();
    deck.count();
}
