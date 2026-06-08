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
