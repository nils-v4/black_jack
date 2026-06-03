enum FaceCards {
    A,
    J,
    Q,
    K,
}

fn value_face_cards(facecard: FaceCards) -> u8 {
    match facecard {
        FaceCards::A => 1,
        FaceCards::J => 11,
        FaceCards::Q => 12,
        FaceCards::K => 13,
    }
}

enum Suits {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

struct Card {
    suit: String,
    number: u32,
}

impl Card {
    fn new(num: u32, suit: String) -> Card {
        Card {
            number: num,
            suit: suit,
        }
    }
}

struct Deck {
    card: Option<Card>,
    amount: Option<u32>,
}

impl Deck {
    fn build_deck(card: Option<Card>, amount: Option<u32>) -> Deck {
        Deck {
            card: card,
            amount: amount,
        }
    }
}

fn main() {
    let suits = vec!["Spades", "Hearts", "Clubs", "Diamonds"];
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    let deck = Deck {
        card: None,
        amount: None,
    };
    let mut amount = 0;
    for suit in &suits {
        for num in &numbers {
            *&mut amount += 1;
            Deck::build_deck(Some(Card::new(*num, suit.to_string())), Some(amount));
        }
    }
}
