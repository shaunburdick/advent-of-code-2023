use clap::Parser;
use std::{fs::read_to_string, path::PathBuf, process, str::FromStr};

mod joker;
mod normal;

#[derive(clap::Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    // Get command line arguments
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(file) = read_to_string(&args.input_file) {
        let mut normal_cards = file
            .lines()
            .flat_map(normal::CamelCardHand::from_str)
            .collect::<Vec<normal::CamelCardHand>>();

        normal_cards.sort();

        let part_1_answer = normal_cards
            .iter()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u32 + 1))
            .sum::<u32>();

        let mut joker_cards = file
            .lines()
            .flat_map(joker::CamelCardHand::from_str)
            .collect::<Vec<joker::CamelCardHand>>();

        joker_cards.sort();

        let part_2_answer = joker_cards
            .iter()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u32 + 1))
            .sum::<u32>();

        println!("Part 1: {}\nPart 2: {}", part_1_answer, part_2_answer);
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}
