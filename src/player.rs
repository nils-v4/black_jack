use crate::GameResult;
use crate::card::*;
use crate::deck::*;

use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Action {
    Hit,
    // Double,
    // Split,
    Stand,
}

pub struct Player {
    name: String,
    money: u32,
    cards: Vec<Card>,
    current_bet: u32,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            money: 100,
            cards: Vec::new(),
            current_bet: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn money(&self) -> u32 {
        self.money
    }

    pub fn cards(&self) {
        for c in &self.cards {
            c.to_string(false);
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

    pub fn process(&mut self, action: Action, deck: &mut Deck) {
        match action {
            Action::Hit => self.take_card(deck),
            // Action::Double => {}
            // Action::Split => {}
            Action::Stand => {}
        }
    }

    pub fn clear_hand(&mut self) {
        self.cards = Vec::new()
    }

    pub fn place_bet(&mut self, amount: u32) {
        self.current_bet = amount;
        self.money -= self.current_bet;
    }

    pub fn settle_bet(&mut self, result: GameResult) {
        match result {
            GameResult::PlayerWin => self.money += self.current_bet * 2,
            GameResult::Draw => self.money += self.current_bet,
            GameResult::DealerWin => (),
        }
        self.current_bet = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_player() {
        let player = Player::new(String::from("Taylor"));
        assert_eq!(player.name, "Taylor");
    }

    #[test]
    fn start_with_100() {
        let player = Player::new(String::from("Taylor"));
        assert_eq!(player.money, 100);
    }

    #[test]
    fn test_player_take_card() {
        let mut player = Player::new(String::from("Taylor"));
        let mut deck = Deck::initialize(1);
        assert_eq!(player.cards.len(), 0);
        player.take_card(&mut deck);
        assert_eq!(player.cards.len(), 1);
    }

    #[test]
    fn test_score_calculation_no_aces() {
        let mut player = Player::new(String::from("Taylor"));
        player
            .cards
            .push(Card::new(FaceCards::Value(10), &Suits::Hearts));
        player.cards.push(Card::new(FaceCards::K, &Suits::Spades));
        assert_eq!(player.calculate_score(), 20);
    }

    #[test]
    fn test_score_calculation_one_ace_under_21() {
        let mut player = Player::new(String::from("Taylor"));
        player
            .cards
            .push(Card::new(FaceCards::Value(9), &Suits::Hearts));
        player.cards.push(Card::new(FaceCards::A, &Suits::Spades));
        assert_eq!(player.calculate_score(), 20);
    }

    #[test]
    fn test_score_calculation_one_ace_over_21() {
        let mut player = Player::new(String::from("Taylor"));
        player
            .cards
            .push(Card::new(FaceCards::Value(10), &Suits::Hearts));
        player
            .cards
            .push(Card::new(FaceCards::Value(5), &Suits::Clubs));
        player.cards.push(Card::new(FaceCards::A, &Suits::Spades));
        assert_eq!(player.calculate_score(), 16);
    }

    #[test]
    fn test_score_calculation_two_aces() {
        let mut player = Player::new(String::from("Taylor"));
        player.cards.push(Card::new(FaceCards::A, &Suits::Hearts));
        player.cards.push(Card::new(FaceCards::A, &Suits::Spades));
        assert_eq!(player.calculate_score(), 12);
    }
}
