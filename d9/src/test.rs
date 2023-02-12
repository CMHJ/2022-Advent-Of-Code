use crate::*;

const TEST_INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[test]
fn test_p1() {
    let n = solve_p1(TEST_INPUT);
    assert_eq!(n, 13);
}