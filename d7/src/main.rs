use d7::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.last().unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let (size, min_size) = solve(input.as_str());
    println!("Part 1: Total size is {}", size);
    println!("Part 2: Smallest dir size is {}", min_size);
}
