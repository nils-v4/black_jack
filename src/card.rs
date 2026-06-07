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
}
