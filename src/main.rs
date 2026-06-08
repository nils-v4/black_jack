mod card;
mod deck;
mod player;
use deck::*;

fn main() {
    let mut deck = Deck::initialize(1);
    deck.shuffle();
    deck.show_deck();
    deck.count();
    let card = deck.take_card();
    card.to_string();
}
