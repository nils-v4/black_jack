use crate::card::*;
use crate::deck::*;

pub struct Dealer {
    cards: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Self {
        Dealer { cards: Vec::new() }
    }

    pub fn cards(&self, hide_card: bool) {
        for (i, c) in self.cards.iter().enumerate() {
            let is_hidden = hide_card && i == 1;
            c.to_string(is_hidden);
        }
    }

    pub fn take_card(&mut self, deck: &mut Deck) {
        let card = deck.take_card();
        self.cards.push(card)
    }

    pub fn calculate_score(&self, hide_second: bool) -> u32 {
        let cards_for_score_count = if hide_second && self.cards.len() >= 2 {
            &self.cards[0..1]
        } else {
            &self.cards[..]
        };
        let mut score: u32 = cards_for_score_count
            .iter()
            .map(|c| c.value_of_card() as u32)
            .sum();
        let mut ace_count = cards_for_score_count.iter().filter(|c| c.is_ace()).count();

        while score > 21 && ace_count > 0 {
            score -= 10;
            ace_count -= 1;
        }

        score
    }

    pub fn play(&mut self, deck: &mut Deck) {
        while self.calculate_score(false) < 17 {
            self.take_card(deck);
        }
    }

    pub fn clear_hand(&mut self) {
        self.cards = Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dealer = Dealer::new();
        assert_eq!(dealer.cards.len(), 0);
        assert_eq!(dealer.calculate_score(false), 0);
        assert_eq!(dealer.calculate_score(true), 0);
    }

    #[test]
    fn test_take_card() {
        let mut dealer = Dealer::new();
        let mut deck = Deck::initialize(1);
        assert_eq!(dealer.cards.len(), 0);
        dealer.take_card(&mut deck);
        assert_eq!(dealer.cards.len(), 1);
    }

    #[test]
    fn test_calculate_score_no_aces() {
        let mut dealer = Dealer::new();
        dealer
            .cards
            .push(Card::new(FaceCards::Value(10), &Suits::Hearts));
        dealer.cards.push(Card::new(FaceCards::K, &Suits::Spades));
        assert_eq!(dealer.calculate_score(false), 20);
        assert_eq!(dealer.calculate_score(true), 10);
    }

    #[test]
    fn test_calculate_score_one_ace_under_21() {
        let mut dealer = Dealer::new();
        dealer
            .cards
            .push(Card::new(FaceCards::Value(6), &Suits::Hearts));
        dealer.cards.push(Card::new(FaceCards::A, &Suits::Spades));
        assert_eq!(dealer.calculate_score(false), 17);
        assert_eq!(dealer.calculate_score(true), 6);
    }

    #[test]
    fn test_calculate_score_one_ace_over_21() {
        let mut dealer = Dealer::new();
        dealer
            .cards
            .push(Card::new(FaceCards::Value(10), &Suits::Hearts));
        dealer
            .cards
            .push(Card::new(FaceCards::Value(5), &Suits::Clubs));
        dealer.cards.push(Card::new(FaceCards::A, &Suits::Spades));
        assert_eq!(dealer.calculate_score(false), 16);
        assert_eq!(dealer.calculate_score(true), 10);
    }

    #[test]
    fn test_calculate_score_two_aces() {
        let mut dealer = Dealer::new();
        dealer.cards.push(Card::new(FaceCards::A, &Suits::Hearts));
        dealer.cards.push(Card::new(FaceCards::A, &Suits::Spades));
        assert_eq!(dealer.calculate_score(false), 12);
        assert_eq!(dealer.calculate_score(true), 11);
    }

    #[test]
    fn test_play() {
        let mut dealer = Dealer::new();
        let mut deck = Deck::initialize(1);
        dealer.play(&mut deck);
        assert!(dealer.calculate_score(false) >= 17);
    }
}
