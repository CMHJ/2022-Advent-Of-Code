/*!
--- Day 9: Rope Bridge ---

 */

#[cfg(test)]
mod test;

struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn as_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

pub fn solve_p1(input: &str) -> usize {
    //! Returns unique positions Tail has visited
    let unique_positions = parse_input(input);

    unique_positions.len()
}

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    let mut unique_positions: Vec<(isize, isize)> = Vec::from([(0, 0)]);
    let mut h = Position { x: 0, y: 0 };
    let mut t = Position { x: 0, y: 0 };

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let direction: char = words.get(0).unwrap().chars().nth(0).unwrap();
        let steps: usize = words.get(1).unwrap().parse().unwrap();

        for _ in 0..steps {
            match direction {
                'L' => {
                    h.x -= 1;
                    if (h.x - t.x).abs() > 1 {
                        t.x -= 1;
                        t.y = h.y;
                    }
                }
                'R' => {
                    h.x += 1;
                    if (h.x - t.x).abs() > 1 {
                        t.x += 1;
                        t.y = h.y;
                    }
                }
                'U' => {
                    h.y += 1;
                    if (h.y - t.y).abs() > 1 {
                        t.y += 1;
                        t.x = h.x;
                    }
                }
                'D' => {
                    h.y -= 1;
                    if (h.y - t.y).abs() > 1 {
                        t.y -= 1;
                        t.x = h.x;
                    }
                }
                _ => {
                    panic!("Unexpected direction!");
                }
            }

            if unique_positions.contains(&t.as_tuple()) == false {
                unique_positions.push(t.as_tuple());
            }
        }

    }

    // println!("Unique positions: {}", unique_positions.len());
    unique_positions
}
