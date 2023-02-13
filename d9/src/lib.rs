/*!
--- Day 9: Rope Bridge ---
// if abs greater than 1
// move next link in line with leader link
// propagate until abs is 1

 */

#[cfg(test)]
mod test;

const KNOTS: usize = 10;

#[derive(Default, Clone, Copy)]
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
                }
                'R' => {
                    h.x += 1;
                }
                'U' => {
                    h.y += 1;
                }
                'D' => {
                    h.y -= 1;
                }
                _ => {
                    panic!("Unexpected direction!");
                }
            }

            // Update tail
            let diff_x = h.x - t.x;
            let diff_y = h.y - t.y;
            // If head is moving on x axis and tail needs to update
            if diff_x.abs() > 1 {
                if diff_x.is_positive() {
                    t.x += 1
                }
                else {
                    t.x -= 1
                }
                t.y += diff_y;
            }
            // If head is moving on y axis and tail needs to update
            if diff_y.abs() > 1 {
                if diff_y.is_positive() {
                    t.y += 1
                }
                else {
                    t.y -= 1
                }
                t.x += diff_x;
            }

            if unique_positions.contains(&t.as_tuple()) == false {
                unique_positions.push(t.as_tuple());
            }
        }
    }

    // println!("Unique positions: {}", unique_positions.len());
    unique_positions.len()
}

pub fn solve_p2(input: &str) -> usize {
    let mut unique_positions: Vec<(isize, isize)> = Vec::from([(0, 0)]);
    let mut knot_positions: [Position; KNOTS] = [Position::default(); KNOTS];

    // For step in steps
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let direction: char = words.get(0).unwrap().chars().nth(0).unwrap();
        let steps: usize = words.get(1).unwrap().parse().unwrap();

        // Move head
        // Calculate head relative to next link
        // If next link needs to move
        // Propagate movement until movement condition is no longer satisfied
        for knot in knot_positions {
            // println!("{:?}", p.as_tuple());
        }
        // Continue
    }

    0
}

fn propagate_movement(leader: &Position, follower: &mut Position) {
    let diff_x = leader.x - follower.x;
    let diff_y = leader.y - follower.y;
    if diff_x.abs() > 1 {
        if diff_x.is_positive() {
            follower.x = leader.x - 1;
        }
        else {
            follower.x = leader.x + 1;
        }
        follower.y = leader.y;
    }
    // else if diff_y
}
