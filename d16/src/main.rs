use d16::*;

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 || args.len() > 2 {
        eprintln!("Please supply input text file path");
        std::process::exit(1);
    }

    let path = args.remove(0);
    let mut input = std::fs::read_to_string(path).expect("Unable to read file from supplied path");

    let score_max = solve_d16(&mut input);
    println!("Max score is: {}", score_max);
}
