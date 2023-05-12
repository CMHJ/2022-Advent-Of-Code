#[cfg(test)]
mod test;

const SCREEN_WIDTH: usize = 40;

pub fn solve(input: &str) -> i32 {
    let mut total_cycles: i32 = 0;
    let mut x: i32 = 1;
    let mut sum: i32 = 0;
    let mut crt_lines: Vec<[bool; SCREEN_WIDTH]> = Vec::new();
    let mut current_line = [false; SCREEN_WIDTH];
    let mut line_index: usize = 0;

    // iterate over instructions
    for ins_raw in input.lines() {
        // parse instruction
        let words: Vec<&str> = ins_raw.split_whitespace().collect();
        let ins = *words.get(0).unwrap();
        let num: Option<i32> = match words.get(1) {
            Some(n) => n.parse().ok(),
            None => None,
        };

        // get cycles to execute
        let cycles = match ins {
            "noop" => 1,
            "addx" => 2,
            _ => panic!("Unexpected instruction!"),
        };

        // execute cycles
        for _ in 0..cycles {
            total_cycles += 1;

            // check for sprite and draw if visible
            if sprite_visible(x, line_index as u32) {
                current_line[line_index] = true;
            }
            line_index += 1;

            // push line when it is finished and reinitialise line buffer
            if line_index >= SCREEN_WIDTH {
                crt_lines.push(current_line);
                current_line = [false; SCREEN_WIDTH];
                line_index = 0;
            }

            // check from 20 in increments of screen width
            if (total_cycles - 20) % SCREEN_WIDTH as i32 == 0 {
                sum += total_cycles * x;
            }
        }

        // if it is an add instruction do the add
        if let Some(n) = num {
            x += n;
        }
    }

    print_lines(&crt_lines);

    sum
}

fn sprite_visible(x: i32, line_index: u32) -> bool {
    // check if any of the pixels of the sprite is at the same position as the
    // currently rendering pixel

    // early exit if x cannot possibly be seen
    if x < -1 {
        return false;
    }

    for pos in (x - 1)..=(x + 1) {
        if pos == line_index as i32 {
            return true;
        }
    }

    false
}

// used to parse test case text
fn parse_render(input: &str) -> Vec<[bool; SCREEN_WIDTH]> {
    let mut crt_lines: Vec<[bool; SCREEN_WIDTH]> = Vec::new();
    let mut current_line = [false; SCREEN_WIDTH];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    current_line[i] = true;
                }
                _ => {}
            }
        }
        crt_lines.push(current_line);
        current_line = [false; SCREEN_WIDTH];
    }

    for line in &crt_lines {
        println!("{:?}", line);
    }
    crt_lines
}

fn print_lines(lines: &Vec<[bool; SCREEN_WIDTH]>) {
    println!("Part 2:");
    for line in lines {
        for c in line {
            match c {
                false => print!("."),
                true => print!("#"),
            }
        }
        println!();
    }
}
