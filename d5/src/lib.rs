#[cfg(test)]
mod test;

use regex::Regex;

struct Move {
    stack: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: String) {
    // Create regex for matching groups of numbers
    let regex_num = Regex::new(r"[0-9]+").expect("Could not create regex");

    // Split based on double newline
    // First part is cargo stack visualisation
    // Second part is list of moves to make
    let mut parts: Vec<&str> = input.split("\n\n").collect();
    // println!("{:?}", &parts);
    let mut stack_visualisation_raw: Vec<&str> = parts.remove(0).lines().collect();
    let rearrangement_procedure_raw = parts;
    // println!("{:?}", stack_visualisation_raw);

    // Find number of columns
    let column_numbers_raw = String::from(stack_visualisation_raw.pop().unwrap());
    let regex_matches: Vec<&str> = regex_num
        .captures_iter(&column_numbers_raw)
        .map(|x| x.get(0).unwrap().as_str())
        .collect();
    let num_columns = regex_matches.len();

    // Create variable for storing parsed stack information
    let mut stack_list: Vec<Vec<char>> = Vec::with_capacity(num_columns);
    for _ in 0..num_columns {
        stack_list.push(Vec::new());
    }
    assert!(stack_list.len() == num_columns);

    // Parse stack visualisation into stack_list which is variable array of
    // stacks, both of which will be type Vector, just a conceptual
    // consideration
    for row in stack_visualisation_raw {
        // Column text is always spaced the same so just iterate start index
        // at 1 and add 4 to check for another column
        let mut i: usize = 0;
        let mut n: usize = 1;
        while let Some(c) = row.chars().nth(n) {
            if c >= 'A' && c <= 'Z' {
                if let Some(v) = stack_list.get_mut(i) {
                    v.push(c);
                }
            }

            i += 1; // Increment list index
            n += 4; // Increment character check index
        }
    }

    // Debug
    // for m in regex_matches {
    //     println!("{}", m);
    // }
    println!("Total {}", num_columns);
    println!("Stack List:\n{:#?}", stack_list);

    // return TBD
}

pub fn solve_part_1(input: String) {
    parse_input(input);
}
