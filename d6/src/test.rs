use crate::*;
use std::iter::zip;

const TEST_INPUTS: [&str; 5] = [
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
    "bvwbjplbgvbhsrlpgdmjqwftvncz",
    "nppdvjthqldpwncqszvftbrmjlhg",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
];
const TEST_ANSWERS_PART_1: [usize; 5] = [7, 5, 6, 10, 11];
const TEST_ANSWERS_PART_2: [usize; 5] = [19, 23, 23, 29, 26];

#[test]
fn test_part_1() {
    for (input, ans) in zip(TEST_INPUTS, TEST_ANSWERS_PART_1) {
        assert_eq!(solve(String::from(input), PART_1_MARKER_LEN), ans);
    }
}

#[test]
fn test_part_2() {
    for (input, ans) in zip(TEST_INPUTS, TEST_ANSWERS_PART_2) {
        assert_eq!(solve(String::from(input), PART_2_MARKER_LEN), ans);
    }
}
