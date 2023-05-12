use crate::*;

mod test_input;
use test_input::*;

#[test]
fn test_p1(){
    let total = solve(TEST_INPUT_1);
    assert_eq!(total, 13140);
}
