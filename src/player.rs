use crate::card::*;
use crate::deck::*;

enum Action {
    Hit,
    Double,
    Split,
    Stand,
}

pub struct Player {
    name: String,
    money: i32,
    cards: Vec<Card>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            money: 100,
            cards: Vec::new(),
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
