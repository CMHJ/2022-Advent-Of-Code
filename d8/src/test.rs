use crate::*;

const TEST_INPUT: &str = r"30373
25512
65332
33549
35390";

#[test]
fn test_p1() {
    let n = solve_p1(TEST_INPUT);
    assert_eq!(n, 21);
}

#[test]
fn test_p2_1() {
    let tree_map = parse_input(TEST_INPUT);
    let score = calculate_scenic_score(&tree_map, 1, 2);
    assert_eq!(score, 4);
}

#[test]
fn test_p2_2() {
    let tree_map = parse_input(TEST_INPUT);
    let score = calculate_scenic_score(&tree_map, 3, 2);
    assert_eq!(score, 8);
}