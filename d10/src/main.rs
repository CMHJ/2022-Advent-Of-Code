use d10::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.last().unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let n = solve(input.as_str());
    println!("Part 1: Total is {}", n);
}
