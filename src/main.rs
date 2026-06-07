mod card;
mod deck;
//use card::*;
use deck::*;

fn main() {
    let mut deck = Deck::initialize(1);
    deck.shuffle();
    deck.show_deck();
    deck.count();
}
