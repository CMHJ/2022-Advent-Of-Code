#[cfg(test)]
mod test;

pub fn solve_p1(input: &str) -> i32 {
    let mut total_cycles: i32 = 0;
    let mut x: i32 = 1;
    let mut sum: i32 = 0;
    let mut crt_lines: Vec<[bool; 40]> = Vec::new();
    let mut current_line: [bool; 40] = [false; 40];

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let ins = *words.get(0).unwrap();
        let num: Option<i32> = match words.get(1) {
            Some(n) => n.parse().ok(),
            None => None
        };

        let cycles = match ins {
            "noop" => 1,
            "addx" => 2,
            _ => panic!("Unexpected instruction!")
        };

        for _ in 0..cycles {
            total_cycles += 1;



            if (total_cycles - 20) % 40 == 0 {
                sum += total_cycles * x;

            }
        }
        if let Some(n) = num {
            x += n;
        }
    }

    sum
}

fn sprite_visible(x: &i32, line_index: &u32) -> bool {
    for pos in (x-1)..=(x+1) {
        if pos == line_index {
            return true;
        }
    }

    false
}

fn parse_render(input: &str) -> Vec<[bool; 40]> {
    let mut crt_lines: Vec<[bool; 40]> = Vec::new();
    let mut current_line = [false; 40];

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
        current_line = [false; 40];
    }

    for line in &crt_lines {
        println!("{:?}", line);
    }
    crt_lines
}