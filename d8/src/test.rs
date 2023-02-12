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
