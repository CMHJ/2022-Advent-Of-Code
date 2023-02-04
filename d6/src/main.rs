use d6::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.last().unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let pos = solve(input.clone(), PART_1_MARKER_LEN);
    println!("Part 1: Marker position is {}", pos);

    let pos = solve(input, PART_2_MARKER_LEN);
    println!("Part 2: Message is {}", pos);
}
