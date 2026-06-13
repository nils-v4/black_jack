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
        let mut score = 0;
        let mut aces = 0;
        for c in &self.cards {
            if c.value_of_card() == 11 {
                aces += 1;
            }
            score += c.value_of_card();
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
}
