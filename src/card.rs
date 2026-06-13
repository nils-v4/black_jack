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
        let value = match &self.number {
            FaceCards::Value() => value,
            FaceCards::A => 11,
            FaceCards::J => 10,
            FaceCards::Q => 10,
            FaceCards::K => 10,
        };
        value
    }

    pub fn to_string(&self) {
        let symbol = match self.suit {
            Suits::Diamonds => "♦",
            Suits::Hearts => "♥",
            Suits::Clubs => "♣",
            Suits::Spades => "♠",
        };
        println!(
            "|{:?}  |\n| {:?} |\n|  {:?}",
            self.number, symbol, self.number
        );
    }
}
