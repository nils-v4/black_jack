use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone)]
pub enum FaceCards {
    Value(u8),
    A,
    J,
    Q,
    K,
}

#[derive(EnumIter, Debug, Clone)]
pub enum Suits {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Debug)]
pub struct Card {
    number: FaceCards,
    suit: Suits,
}

impl Card {
    pub fn new(num: FaceCards, suit: &Suits) -> Self {
        Card {
            number: num.clone(),
            suit: suit.clone(),
        }
    }

    pub fn value_of_card(&self) -> u8 {
        match &self.number {
            FaceCards::Value(val) => *val,
            FaceCards::A => 11,
            FaceCards::J => 10,
            FaceCards::Q => 10,
            FaceCards::K => 10,
        }
    }

    pub fn is_ace(&self) -> bool {
        matches!(self.number, FaceCards::A)
    }

    pub fn to_string(&self) {
        let symbol = match self.suit {
            Suits::Diamonds => "♦",
            Suits::Hearts => "♥",
            Suits::Clubs => "♣",
            Suits::Spades => "♠",
        };
        let val_str = match &self.number {
            FaceCards::Value(val) => val.to_string(),
            FaceCards::A => "A".to_string(),
            FaceCards::J => "J".to_string(),
            FaceCards::Q => "Q".to_string(),
            FaceCards::K => "K".to_string(),
        };
        println!("┌─────────┐");
        println!("│ {:<2}      │", val_str);
        println!("│    {}    │", symbol);
        println!("│      {:>2} │", val_str);
        println!("└─────────┘");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_values() {
        let card_a = Card::new(FaceCards::A, &Suits::Hearts);
        let card_k = Card::new(FaceCards::K, &Suits::Spades);
        let card_q = Card::new(FaceCards::Q, &Suits::Diamonds);
        let card_j = Card::new(FaceCards::J, &Suits::Clubs);
        let card_5 = Card::new(FaceCards::Value(5), &Suits::Hearts);

        assert_eq!(card_a.value_of_card(), 11);
        assert_eq!(card_k.value_of_card(), 10);
        assert_eq!(card_q.value_of_card(), 10);
        assert_eq!(card_j.value_of_card(), 10);
        assert_eq!(card_5.value_of_card(), 5);
    }

    #[test]
    fn test_is_ace() {
        let card_a = Card::new(FaceCards::A, &Suits::Hearts);
        let card_10 = Card::new(FaceCards::Value(10), &Suits::Spades);

        assert!(card_a.is_ace());
        assert!(!card_10.is_ace());
    }
}
