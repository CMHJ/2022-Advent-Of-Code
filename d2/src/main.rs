mod rps;

use std::{process::exit, fs};
use rps::eval_result;

use crate::rps::eval_result_2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 || args.len() > 2 {
        eprintln!("Please supply path to input text");
        exit(1);
    }
    let path = args.get(1).unwrap();
    let contents = fs::read_to_string(path).unwrap();

    let rounds: Vec<&str> = contents.split('\n').collect();
    let mut total_score: u64 = 0;
    let mut total_score_2: u64 = 0;

    for round in rounds {
        let choices: Vec<&str> = round.split(' ').collect();
        let a: char = choices.get(0).unwrap().chars().next().expect("Failed to parse char");
        let b: char = choices.get(1).unwrap().chars().next().expect("Failed to parse char");

        let score = eval_result(&a, &b);
        total_score += score as u64;

        let score_2 = eval_result_2(&a, &b);
        total_score_2 += score_2 as u64;
    }

    println!("Total score if following strategy: {:?}", total_score);
    println!("Total score if following actual strategy: {:?}", total_score_2);
}
