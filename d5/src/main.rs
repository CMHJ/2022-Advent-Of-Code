use d5::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.last().unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let msg = solve_part_1(input.clone());
    println!("Part 1: Message is {}", msg);

    let msg = solve_part_2(input);
    println!("Part 2: Message is {}", msg);
}
