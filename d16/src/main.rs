use d16::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.last().unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let score = solve_p1(input.clone());
    println!("Part 1: Max score is {}", score);
}