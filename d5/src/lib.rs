#[cfg(test)]
mod test;

use regex::Regex;

struct Move {
    stack: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: String) {
    let regex_num = Regex::new(r"[0-9]+").expect("Could not create regex");

    // Split based on double newline
    // First is cargo stack visualisation
    // Second is moves to make
    let mut parts: Vec<&str> = input.split("\n\n").collect();
    let mut stack_visualisation_raw: Vec<&str> = parts.remove(0).lines().collect();
    println!("{:?}", stack_visualisation_raw);
    let column_numbers_raw = String::from(stack_visualisation_raw.pop().unwrap());

    // Find number of columns
    let regex_matches: Vec<&str> = regex_num
        .captures_iter(&column_numbers_raw)
        .map(|x| x.get(0).unwrap().as_str())
        .collect();
    let columns = regex_matches.len();
    for m in regex_matches {
        println!("{}", m);
    }
    println!("Total {}", columns);

    // return TBD
}

pub fn solve_part_1(input: String) {
    parse_input(input);
}
