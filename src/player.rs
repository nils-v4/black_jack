use crate::card::*;
use crate::deck::*;

pub struct Player {
    name: String,
    money: i32,
    cards: Vec<Option<Card>>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            money: 100,
            cards: vec![None],
        }
    }

    // need to fix this Some(card) push,
    pub fn take_card(&mut self, deck: &mut Deck) {
        let card = deck.take_card();
        self.cards.push(Some(card))
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
