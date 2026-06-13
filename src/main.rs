mod card;
mod deck;
mod player;
use deck::*;
use std::io::*;

fn main() {
    let mut deck = Deck::initialize(1);
    deck.shuffle();
    deck.show_deck();
    deck.count();
    let card = deck.take_card();
    card.to_string();

    let mut input: u32;
    std::io::stdin().read_line(&mut input);
}
