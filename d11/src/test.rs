use crate::*;

mod test_input;
use test_input::*;

#[test]
fn test_p1(){
    let mut m_vec = parse_input(TEST_INPUT);
    // println!("{:#?}", &m_vec);
    run_rounds(&mut m_vec);
    println!();
    println!("{:#?}", &m_vec);
}
