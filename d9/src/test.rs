use crate::*;

const TEST_INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

const TEST_INPUT_2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

#[test]
fn test_p1() {
    let n = solve_p1(TEST_INPUT);
    assert_eq!(n, 13);
}

#[test]
fn test_p2_1() {
    let n = solve_p2(TEST_INPUT);
    assert_eq!(n, 1);
}

#[test]
fn test_p2_2() {
    let n = solve_p2(TEST_INPUT_2);
    assert_eq!(n, 36);
}