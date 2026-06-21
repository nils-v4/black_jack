mod card;
mod dealer;
mod deck;
mod player;
use dealer::*;
use deck::*;
use player::*;

fn main() {
    let mut deck = Deck::initialize(1);
    deck.shuffle();

    let mut dealer = Dealer::new();

    println!("Enter name:");
    let mut input_name = String::new();
    std::io::stdin()
        .read_line(&mut input_name)
        .expect("Failed to process name");
    let mut player1 = Player::new(input_name.trim().to_string());

    loop {
        std::process::Command::new("clear").status().ok();
        loop {
            println!("Enter bet: (current amount is {})", player1.money());
            let mut input_bet = String::new();
            std::io::stdin()
                .read_line(&mut input_bet)
                .expect("Failed to process bet");

            match input_bet.trim().parse::<u32>() {
                Ok(amount) => {
                    if amount == 0 {
                        println!("amount must be greater than 0");
                    } else if amount > player1.money() {
                        println!("you dont have enough for that");
                    } else {
                        player1.place_bet(amount);
                        break;
                    }
                }
                Err(_) => {
                    println!("invalid input");
                }
            }
        }

        for _ in 1..=2 {
            dealer.take_card(&mut deck);
            player1.take_card(&mut deck);
        }
        show_game_state(&player1, &dealer, true);
        loop {
            println!("What are you going to do? hit / stand");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.to_lowercase().trim() {
                "h" | "hit" => {
                    player1.process(Action::Hit, &mut deck);
                    if player1.calculate_score() > 21 {
                        break;
                    }
                }
                "s" | "stand" => {
                    player1.process(Action::Stand, &mut deck);
                    break;
                }
                _ => continue,
            }
            show_game_state(&player1, &dealer, false);
        }

        if player1.calculate_score() <= 21 {
            dealer.play(&mut deck);
            show_game_state(&player1, &dealer, false);
        } else {
            show_game_state(&player1, &dealer, false);
            println!("you busted");
        }

        let result = determine_winner(&player1, &dealer);
        match &result {
            GameResult::PlayerWin => println!("winner: {}", player1.name()),
            GameResult::DealerWin => println!("winner: dealer"),
            GameResult::Draw => println!("Draw!"),
        }
        player1.settle_bet(result);

        player1.clear_hand();
        dealer.clear_hand();

        println!("Current money is: {}", player1.money());

        if player1.money() <= 0 {
            println!("You are out of money and are getting kicked out of the casino!");
            break;
        }
        println!("\nPress Enter to start next round...");
        let mut next_round = String::new();
        std::io::stdin()
            .read_line(&mut next_round)
            .expect("failed to read input");
    }
}

fn show_game_state(player: &Player, dealer: &Dealer, hide_dealer_card: bool) {
    std::process::Command::new("clear").status().ok();
    println!("\n--- dealer hand ---");
    println!("Score: {}", dealer.calculate_score(hide_dealer_card));
    dealer.cards(hide_dealer_card);
    println!("");
    println!(
        "\n--- {} hand --- money: {} ---",
        player.name(),
        player.money()
    );
    println!("Score: {}", player.calculate_score());
    player.cards();
    println!("");
}

pub enum GameResult {
    PlayerWin,
    DealerWin,
    Draw,
}

fn determine_winner(player: &Player, dealer: &Dealer) -> GameResult {
    let player_score = player.calculate_score();
    let dealer_score = dealer.calculate_score(false);

    if player_score > 21 {
        GameResult::DealerWin
    } else if dealer_score > 21 {
        GameResult::PlayerWin
    } else if player_score > dealer_score {
        GameResult::PlayerWin
    } else if dealer_score > player_score {
        GameResult::DealerWin
    } else {
        GameResult::Draw
    }
}
