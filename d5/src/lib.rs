#[cfg(test)]
mod test;

use regex::Regex;

#[derive(Debug)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: String) -> (Vec<Vec<char>>, Vec<Move>) {
    // Create regex for matching groups of numbers
    let regex_num = Regex::new(r"[0-9]+").expect("Could not create regex");

    // Split based on double newline
    // First part is cargo stack visualisation
    // Second part is list of moves to make
    let mut parts: Vec<&str> = input.split("\n\n").collect();
    let mut stack_visualisation_raw: Vec<&str> = parts.remove(0).lines().collect();
    let stack_procedure_raw = parts.remove(0).lines().collect();

    // Find number of columns
    let column_numbers_raw = String::from(stack_visualisation_raw.pop().unwrap());
    let regex_matches: Vec<&str> = regex_num
        .captures_iter(&column_numbers_raw)
        .map(|x| x.get(0).unwrap().as_str())
        .collect();
    let num_columns = regex_matches.len();

    let stack_list = parse_stack_visualisation(stack_visualisation_raw, num_columns);
    let move_list = parse_stack_procedure(stack_procedure_raw);

    return (stack_list, move_list);
}

fn parse_stack_visualisation(
    mut stack_visualisation_raw: Vec<&str>,
    num_columns: usize,
) -> Vec<Vec<char>> {
    // Create variable for storing parsed stack information
    let mut stack_list: Vec<Vec<char>> = Vec::with_capacity(num_columns);
    for _ in 0..num_columns {
        stack_list.push(Vec::new());
    }
    assert!(stack_list.len() == num_columns);

    // Reverse lines so they are push to stack from bottom up in correct order
    stack_visualisation_raw.reverse();

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

    return stack_list;
}

fn parse_stack_procedure(stack_procedure_raw: Vec<&str>) -> Vec<Move> {
    // Create regex for matching groups of numbers
    let regex_num = Regex::new(r"[0-9]+").expect("Could not create regex");
    let mut move_list: Vec<Move> = Vec::with_capacity(stack_procedure_raw.len());

    for move_raw in stack_procedure_raw {
        let regex_matches: Vec<&str> = regex_num
            .captures_iter(&move_raw)
            .map(|x| x.get(0).unwrap().as_str())
            .collect();

        let n = regex_matches.get(0).unwrap().parse::<usize>().unwrap();
        let from = regex_matches.get(1).unwrap().parse::<usize>().unwrap();
        let to = regex_matches.get(2).unwrap().parse::<usize>().unwrap();
        move_list.push(Move { n, from, to });
    }

    return move_list;
}

fn run_stack_procedure(mut stack_list: Vec<Vec<char>>, move_list: Vec<Move>) -> String {
    for m in move_list {
        for _ in 0..m.n {
            let stack_from = stack_list.get_mut(m.from - 1).unwrap();
            let c = stack_from.pop().unwrap();
            let stack_to = stack_list.get_mut(m.to - 1).unwrap();
            stack_to.push(c);
        }
    }

    // Construct message for result
    let mut msg = String::new();
    for v in stack_list {
        msg.push(v.last().unwrap().clone());
    }

    return msg;
}

pub fn solve_part_1(input: String) -> String {
    let (stack_list, move_list) = parse_input(input);
    let msg = run_stack_procedure(stack_list, move_list);

    return msg;
}
