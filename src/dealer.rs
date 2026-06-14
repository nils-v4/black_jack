use crate::card::*;
use crate::deck::*;

pub struct Dealer {
    cards: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Self {
        Dealer { cards: Vec::new() }
    }

    pub fn cards(&self) {
        for c in &self.cards {
            c.to_string();
        }
    }

    pub fn take_card(&mut self, deck: &mut Deck) {
        let card = deck.take_card();
        self.cards.push(card)
    }

    pub fn calculate_score(&self) -> u32 {
        let mut score: u32 = self.cards.iter().map(|c| c.value_of_card() as u32).sum();
        let mut ace_count = self.cards.iter().filter(|c| c.is_ace()).count();

        while score > 21 && ace_count > 0 {
            score -= 10;
            ace_count -= 1;
        }

        score
    }

    pub fn play(&mut self, deck: &mut Deck) {
        while self.calculate_score() < 17 {
            self.take_card(deck);
        }
    }
}
