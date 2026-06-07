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

#[derive(Debug)]
struct Card {
    suit: String,
    number: u32,
}

impl Card {
    fn new(num: u32, suit: String) -> Self {
        Card {
            number: num,
            suit: suit,
        }
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn add_card(&mut self, card: Card) {
        let _ = &mut self.cards.push(card);
    }

    fn show_deck(&self) {
        for c in &self.cards {
            println!("{:?}", c);
        }
    }
}

fn main() {
    let suits = vec!["Spades", "Hearts", "Clubs", "Diamonds"];
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    let mut deck = Deck { cards: Vec::new() };
    for suit in &suits {
        for num in &numbers {
            let card = Card::new(*num, (&suit).to_string());
            deck.add_card(card);
        }
    }
    let _ = &mut deck.show_deck();
}
