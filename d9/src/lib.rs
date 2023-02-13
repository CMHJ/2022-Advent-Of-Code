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
            head_step(&mut h, direction);

            // Update tail
            let position_transform = next_link_step(&h, &t);
            t.x += position_transform.0;
            t.y += position_transform.1;

            if unique_positions.contains(&t.as_tuple()) == false {
                unique_positions.push(t.as_tuple());
            }
        }
    }

    // println!("Unique positions: {}", unique_positions.len());
    unique_positions.len()
}

fn head_step(h: &mut Position, direction: char) {
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
}

fn next_link_step(h: &Position, t: &Position) -> (isize, isize) {
    //! Returns transformation for next link in the rope after "head" has already moved
    // Get vectors from "tail" to "head"
    let diff_x = h.x - t.x;
    let diff_y = h.y - t.y;
    let mut position_transform = (0, 0);
    // If head has moved sufficiently far on x axis and tail needs to update
    if diff_x.abs() > 1 {
        if diff_x.is_positive() {
            position_transform.0 = 1;
        } else {
            position_transform.0 = -1;
        }
        // Clamp is to account for diagonal movement of previous link where diff_y will be 2
        position_transform.1 = diff_y.clamp(-1, 1);
    }
    // If head has moved sufficiently far on y axis and tail needs to update
    else if diff_y.abs() > 1 {
        if diff_y.is_positive() {
            position_transform.1 = 1;
        } else {
            position_transform.1 = -1;
        }
        // Clamp is to account for diagonal movement of previous link where diff_x will be 2
        position_transform.0 = diff_x.clamp(-1, 1);
    }

    position_transform
}

pub fn solve_p2(input: &str) -> usize {
    let mut unique_positions: Vec<(isize, isize)> = Vec::from([(0, 0)]);
    let mut knot_positions: [Position; KNOTS] = [Position::default(); KNOTS];

    // For step in steps
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let direction: char = words.get(0).unwrap().chars().nth(0).unwrap();
        let steps: usize = words.get(1).unwrap().parse().unwrap();

        for _ in 0..steps {
            // Move head
            head_step(&mut knot_positions[0], direction);

            // Propagate movement down the knots until movement condition is no
            // longer satisfied
            for tail_index in 1..knot_positions.len() {
                // Split array into two mutable slices so that we can operator
                // on two elements at the same time
                let (first_half, second_half) = knot_positions.split_at_mut(tail_index);
                let h = first_half.last_mut().unwrap();
                let t = second_half.first_mut().unwrap();

                // Calculate head relative to next link
                // and if next link needs to move
                let position_transform = next_link_step(&h, &t);
                // If knot no longer has to move just break as any other knot
                // will also not have to move
                if position_transform.0 == 0 && position_transform.1 == 0 {
                    break;
                }

                // Perform movement
                t.x += position_transform.0;
                t.y += position_transform.1;
                // println!("{:?}", p.as_tuple());
            }

            if unique_positions.contains(&knot_positions.last().unwrap().as_tuple()) == false {
                unique_positions.push(knot_positions.last().unwrap().as_tuple());
            }
        }
    }

    // println!("Unique positions: {}", unique_positions.len());
    unique_positions.len()
}
