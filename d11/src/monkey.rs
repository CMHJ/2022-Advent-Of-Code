pub struct Monkey {
    items: Vec<usize>,
    op: Action,
    op_val: isize,
    test: Action,
    test_val: isize,
    case_true: usize,
    case_false: usize,
}

pub enum Action {
    Add,
    Sub,
    Mul,
    Div,
}
