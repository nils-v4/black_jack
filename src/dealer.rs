use crate::card::*;
use crate::deck::*;

struct Dealer {
    cards: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Self {
        Dealer { cards: Vec::new() }
    }

    pub fn take_card(&mut self, deck: &mut Deck) {
        let card = deck.take_card();
        self.cards.push(card)
    }
}
