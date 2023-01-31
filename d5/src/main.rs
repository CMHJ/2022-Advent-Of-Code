use d5::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.last().unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let score = solve_part_1(input);
    // println!("Part 1: Number of pairs is {}", score);
}
