use crate::*;

const TEST_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

#[test]
fn test_part_1() {
    let score = solve_part_1(String::from(TEST_INPUT));
    assert_eq!(2, score);
}

#[test]
fn test_part_2() {
    let score = solve_part_2(String::from(TEST_INPUT));
    assert_eq!(4, score);
}
