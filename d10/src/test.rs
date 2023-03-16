use crate::*;

mod test_input;
use test_input::*;

#[test]
fn test_p1(){
    let total = solve_p1(TEST_INPUT_1);
    assert_eq!(total, 13140);
}

#[test]
fn test_p2(){
    parse_render(TEST_INPUT_2);
    // let total = solve_p1(TEST_INPUT_1);
    // assert_eq!(total, 13140);
}