use rand::prelude::*;
use std::{cmp::Ordering, io};

#[derive(Clone, Debug)]
struct Card {
    value: u16,
}

impl Card {
    pub fn new(value: u16) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card({})", self.value)
    }
}

fn get_random_card(rng: &mut ThreadRng, cards: &[Card]) -> Card {
    let rand_index = rng.gen_range(0..cards.len());
    cards[rand_index].clone()
}

struct Pocket {
    cards: Vec<Card>,
    total: u16,
}

impl Pocket {
    fn new() -> Self {
        Self {
            cards: Vec::default(),
            total: 0,
        }
    }

    fn push(&mut self, card: Card) {
        self.total += card.value;
        self.cards.push(card);
    }

    fn get_total(&self) -> u16 {
        self.total
    }

    fn exceeds_twenty_one(&self) -> bool {
        self.total > 21
    }
}

fn main() {
    println!("BlackJack 21");
    let mut rng = thread_rng();
    let cards: Vec<Card> = (1..22).map(|v| Card::new(v)).collect();

    let mut player_pocket = Pocket::new();
    let mut cpu_pocket = Pocket::new();

    for i in 0..2 {
        let card = get_random_card(&mut rng, &cards);
        println!("Card {} to CPU: {}", i + 1, card);
        cpu_pocket.push(card);
    }

    for i in 0..2 {
        let card = get_random_card(&mut rng, &cards);
        println!("Card {} to Player: {}", i + 1, card);
        player_pocket.push(card);
    }

    println!(
        "Currently Player's cards sum to: {}",
        player_pocket.get_total()
    );

    println!("{}", "---".repeat(10));

    while !player_pocket.exceeds_twenty_one() {
        println!("Do you wish another card [Y/N]?");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let give_card = buffer.to_lowercase().trim() == "y";

        if give_card {
            let card = get_random_card(&mut rng, &cards);

            println!(
                "Your new card is: {}, new total is: {}",
                &card,
                player_pocket.get_total() + card.value
            );

            player_pocket.push(card);
        } else {
            break;
        }
    }

    if player_pocket.exceeds_twenty_one() {
        println!(
            "Too bad, your new total is {}. Good Luck next time!",
            player_pocket.get_total()
        );

        println!("The CPU had these cards:");
        for card in cpu_pocket.cards.iter() {
            println!("{}", card);
        }

        std::process::exit(1);
    }

    println!("{}", "---".repeat(10));
    println!("The CPU has a total of {}", cpu_pocket.get_total());
    println!("The Player has a total of {}", player_pocket.get_total());

    if cpu_pocket.exceeds_twenty_one() {
        println!("Congratulations! You won the game!");
        std::process::exit(0);
    }

    match player_pocket.get_total().cmp(&cpu_pocket.get_total()) {
        Ordering::Less => {
            println!("Too bad, you lost the game :(");
        }
        Ordering::Equal => {
            println!("Tie, both player and cpu have the same total");
        }

        Ordering::Greater => {
            println!("Congratulations! You won the game!");
        }
    }
}
