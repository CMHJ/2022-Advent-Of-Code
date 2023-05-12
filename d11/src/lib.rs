#[cfg(test)]
mod test;
mod monkey;

use monkey::*;

pub fn solve(input: &str) -> usize {
    // parse input


    0
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let monkey_vec = Vec::new();

    for monkey_text_block in input.split("\n\n"){
        // init vars to construct monkey object

        for line in monkey_text_block.lines() {
            let line_trimmed = line.trim().to_string();
            let words: Vec<&str> = line_trimmed.split(' ').collect();
            match words.get(0) {
                Some(&"Monkey") => {
                    println!("{}", line_trimmed);
                },
                Some(&"Starting") => {

                    println!("{}", line_trimmed);
                },
                Some(&"Operation:") => {

                    println!("{}", line_trimmed);
                },
                Some(&"Test:") => {

                    println!("{}", line_trimmed);
                },
                Some(&"If") => {
                    println!("{}", line_trimmed);

                },
                Some(&&_) => panic!("Encountered Unexpected word when parsing"),
                None => panic!("Encountered None when should have been a word when parsing"),

            }
        }

        // construct and push back monkey
    }

monkey_vec
}