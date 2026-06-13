mod card;
mod deck;
mod player;
//use dealer::*;
use deck::*;
use player::*;
use std::io::*;

fn main() {
    let mut deck = Deck::initialize(1);
    deck.shuffle();
    /*deck.show_deck();
    deck.count();
    let card = deck.take_card();
    card.to_string();*/

    //let dealer = Dealer::new();
    let player1 = Player::new(String::from("nilsv"));

    while true {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let action: String = match input.trim().parse() {
            Ok(act) => act,
            Err(_) => continue,
        };
    }
}
