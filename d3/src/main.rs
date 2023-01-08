use d3::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let input_path = args.last().unwrap();
    let input: String = std::fs::read_to_string(input_path).unwrap();

    let score_total = solve_part_1(input.clone());
    println!("Total score for part 1 is: {}", score_total);

    let score_total = solve_part_2(input);
    println!("Total score for part 2 is: {}", score_total);
}
