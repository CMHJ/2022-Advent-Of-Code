use crate::*;

const TEST_INPUT: &str = r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn test_part_1() {
    let msg = solve_part_1(String::from(TEST_INPUT));
    assert_eq!(msg, "CMZ");
}

#[test]
fn test_part_2() {
    let msg = solve_part_2(String::from(TEST_INPUT));
    assert_eq!(msg, "MCD");
}