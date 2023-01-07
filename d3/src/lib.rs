struct Rucksack {
    compartments: [[usize; 52]; 2],
    // compartment_size: usize
}

impl Rucksack {
    pub fn new(// compartment_size: usize
    ) -> Rucksack {
        Rucksack {
            compartments: [[0; 52]; 2],
            // compartment_size
        }
    }

    fn character_add_count(&mut self, c: char, compartment: usize) {
        // if a-z
        if c >= 'a' && c <= 'z' {
            let c_num = c as usize;
            // Map character to range 0..25
            let c_mapped = c_num - 'a' as usize;

            self.compartments[compartment][c_mapped] += 1;
        }
        // else if is A-Z
        else if c >= 'A' && c <= 'Z' {
            let c_num = c as usize;
            // Map character to range 26-51
            let c_mapped = c_num - 'A' as usize + 26;

            self.compartments[compartment][c_mapped] += 1;
        } else {
            panic!("Encountered unexpected character");
        }
    }

    fn find_max_priority(&self) -> usize {
        for i in (0..=51_usize).rev() {
            if self.compartments[0][i] > 0 && self.compartments[1][i] > 0 {
                return i + 1;
            }
        }
        panic!("No matching number found");
    }
}

fn parse_input(input: String) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    // Iterate over every line
    for l in input.lines() {
        // Count number of characters on line to know when to split in half
        let rs_comp_size = l.len() / 2;
        // Create new Rucksack type for each line
        let mut rucksack = Rucksack::new();

        for (i, c) in l.chars().enumerate() {
            let compartment: usize = if i < rs_comp_size { 0 } else { 1 };
            rucksack.character_add_count(c, compartment)
        }

        rucksacks.push(rucksack);
    }

    rucksacks
}

pub fn solve_part_1(input: String) -> usize {
    // Each character in the text represents a different item
    // Each line of text represents a different rucksack
    // A rucksack has two compartments
    // Each compartment has the same number of items as the other
    // All items of a given type are meant to go into exactly one of the two compartments
    let rucksacks = parse_input(input);

    // Work out total score
    let mut score_total: usize = 0;
    for r in rucksacks {
        let score = r.find_max_priority();
        score_total += score;
    }

    return score_total;
}

pub fn solve_part_2(input: String) -> usize {
    let rucksacks = parse_input(input);

    let mut score_total: usize = 0;

    // Reverse iterate through each rucksack to find common badge
    let mut i: usize = 0;
    while i < rucksacks.len() {
        let group: [&Rucksack; 3] = [
            rucksacks.get(i).unwrap(),
            rucksacks.get(i + 1).unwrap(),
            rucksacks.get(i + 2).unwrap(),
        ];

        for j in (0..=51_usize).rev() {}

        i += 3;
    }

    0
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        let score = solve_part_1(String::from(INPUT));
        assert_eq!(157, score);
    }

    fn test_part_2() {
        let score = solve_part_2(String::from(INPUT));
        assert_eq!(70, score);
    }
}
