#[cfg(test)]
mod test;

// Problem statement
// Each section has unique ID number
// Each Elf is assigned a range of section IDs
// Elves have noticed that assigned section IDs overlap
// Find how many pairs exist where one's section range is completely contained
// in the other

#[derive(Copy, Clone)]
struct Range {
    // min and max section number assigned in range
    min: usize,
    max: usize,
}

impl Default for Range {
    fn default() -> Self {
        Range { min: 0, max: 0 }
    }
}

fn parse_input(input: String) -> Vec<[Range; 2]> {
    let mut range_pairs: Vec<[Range; 2]> = Vec::new();

    for line in input.lines() {
        let mut range_pair = [Range::default(); 2];
        for (i, range_raw) in line.split(',').enumerate() {
            // range raw is of format "X-X" where X is a number
            for (j, num) in range_raw.split('-').enumerate() {
                if j == 0 {
                    range_pair[i].min = num
                        .parse::<usize>()
                        .expect("Could not parse text to number");
                } else {
                    range_pair[i].max = num
                        .parse::<usize>()
                        .expect("Could not parse text to number");
                }
            }
        }

        range_pairs.push(range_pair);
    }

    return range_pairs;
}

pub fn solve_part_1(input: String) -> usize {
    // parse input, return Vec<[Range; 2]>
    let range_pairs = parse_input(input);

    let mut count: usize = 0;
    for r in range_pairs {
        // range 0 contains range 1
        if (r[0].min >= r[1].min && r[0].max <= r[1].max) ||
        // range 1 contains range 0
        (r[1].min >= r[0].min && r[1].max <= r[0].max)
        {
            count += 1;
        }
    }

    return count;
}

pub fn solve_part_2(input: String) -> usize {
    // parse input, return Vec<[Range; 2]>
    let range_pairs = parse_input(input);

    let mut count: usize = 0;
    for r in range_pairs {
        // range 0 min in range 1
        if (r[0].min >= r[1].min && r[0].min <= r[1].max) ||
        // range 0 max in range 1
        (r[0].max >= r[1].min && r[0].max <= r[1].max) ||
        // range 1 min in range 0
        (r[1].min >= r[0].min && r[1].min <= r[0].max) ||
        // range 1 max in range 0
        (r[1].max >= r[0].min && r[1].max <= r[0].max)
        {
            count += 1;
        }
    }

    return count;
}
