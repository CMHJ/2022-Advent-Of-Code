use d16::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>();
    if args.len() == 1 || args.len() > 2 {
        eprintln!("Please supply input text file path");
        std::process::exit(1);
    }

    let path = args.remove(0);
    let input = std::fs::read_to_string(path).expect("Unable to read file from supplied path");

    solve_d16(input);
}
