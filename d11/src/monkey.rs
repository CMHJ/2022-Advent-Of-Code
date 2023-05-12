#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<Item>,
    pub items_inspected: usize,
    pub op: Action,
    pub op_val: isize,
    pub op_type: OpType,
    pub test: Action,
    pub test_val: isize,
    pub monkey_true: usize,
    pub monkey_false: usize,
}

#[derive(Debug)]
pub struct Item {
    pub worry_level: usize,
}

#[derive(Debug)]
pub enum OpType {
    Old,
    Number,
}

#[derive(Debug)]
pub enum Action {
    Add,
    Sub,
    Mul,
    Div,
}

impl Monkey {
    pub fn from_str(monkey_text_block: &str) -> Monkey {
        // init vars to construct monkey object
        let mut items: Vec<Item> = Vec::new();
        let mut op = Action::Add;
        let mut op_val: isize = 0;
        let mut op_type = OpType::Number;
        let mut test = Action::Add;
        let mut test_val: isize = 0;
        let mut monkey_true: usize = 0;
        let mut monkey_false: usize = 0;

        for line in monkey_text_block.lines() {
            let line_trimmed = line.trim().to_string();
            let words: Vec<&str> = line_trimmed.split(' ').collect();
            match words.get(0) {
                Some(&"Monkey") => {
                    // do nothing
                }
                Some(&"Starting") => {
                    let items_raw: Vec<&str> = line_trimmed[16..].split(", ").collect();
                    for item_raw in items_raw {
                        // println!("{}", item_raw);
                        items.push(Item {
                            worry_level: item_raw.parse().expect("Could not parse item number"),
                        });
                    }
                }
                Some(&"Operation:") => {
                    // println!("{}", line_trimmed);
                    let words: Vec<&str> = line_trimmed[21..].split(" ").collect();
                    // for i in items_raw.iter() {
                    //     println!("{}", i);
                    // }
                    op = match words.get(0).unwrap() {
                        &"+" => Action::Add,
                        &"-" => Action::Sub,
                        &"*" => Action::Mul,
                        &"/" => Action::Div,
                        &_ => {
                            panic!("Encountered unexpected operation action when parsing");
                        }
                    };
                    // Set operation value or whether to use the old value
                    match words.get(1).unwrap() {
                        &"old" => {
                            op_type = OpType::Old;
                        }
                        op_val_raw => {
                            op_val = op_val_raw.parse().expect("Could not parse op number")
                        }
                    }
                }
                Some(&"Test:") => {
                    // println!("{}", line_trimmed);
                    let words: Vec<&str> = line_trimmed.split(" ").collect();
                    match words.get(1).unwrap() {
                        &"divisible" => test = Action::Div,
                        &&_ => panic!("Encountered unexpected test action when parsing"),
                    }
                    test_val = words
                        .get(3)
                        .unwrap()
                        .parse()
                        .expect("Could not parse test number");
                }
                Some(&"If") => {
                    // println!("{}", line_trimmed);
                    let words: Vec<&str> = line_trimmed.split(" ").collect();
                    match words.get(1).unwrap() {
                        &"true:" => {
                            monkey_true = words
                                .get(5)
                                .unwrap()
                                .parse()
                                .expect("Could not monkey true number")
                        }
                        &"false:" => {
                            monkey_false = words
                                .get(5)
                                .unwrap()
                                .parse()
                                .expect("Could not monkey true number")
                        }
                        &_ => panic!("Encountered unexpected bool word when parsing"),
                    }
                }
                Some(&&_) => panic!("Encountered unexpected word when parsing"),
                None => panic!("Encountered None when should have been a word when parsing"),
            }
        }

        // println!("{:?}", items);
        // println!("{:?} {} {:?}", op, op_val, op_type);

        Monkey {
            items,
            items_inspected: 0,
            op,
            op_val,
            op_type,
            test,
            test_val,
            monkey_true,
            monkey_false,
        }
    }
}
