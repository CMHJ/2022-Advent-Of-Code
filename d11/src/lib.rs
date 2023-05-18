mod monkey;
#[cfg(test)]
mod test;

use monkey::*;
use std::mem::take;

const NUM_ROUNDS: usize = 20;
const RELIEF_FACTOR: isize = 3;

pub fn solve(input: &str) -> usize {
    let mut m_vec = parse_input(input);
    println!("{:#?}", &m_vec);
    run_rounds(&mut m_vec);
    println!();
    println!("{:#?}", &m_vec);

    // (TODO) count most active monkeys

    0
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkey_vec = Vec::new();

    for monkey_text_block in input.split("\n\n") {
        let monkey = Monkey::from_str(monkey_text_block);
        monkey_vec.push(monkey);
    }

    monkey_vec
}

fn run_rounds(m_vec: &mut Vec<Monkey>) {
    for _ in 0..NUM_ROUNDS {
        for m_idx in 0..m_vec.len() {
            // take the monkey temporarily to avoid multiple mutable refs error
            let mut m: Monkey = take(&mut m_vec[m_idx]);

            // take out all the items
            let items = take(&mut m.items);
            for mut item in items.into_iter() {
                m.items_inspected += 1;

                let second_operand = match m.op_type {
                    OpType::Number => m.op_val,
                    OpType::Old => item.worry_level,
                };

                // monkey inspects item
                item.worry_level = match m.op {
                    Action::Add => item.worry_level + second_operand,
                    Action::Sub => item.worry_level - second_operand,
                    Action::Mul => item.worry_level * second_operand,
                    Action::Div => item.worry_level / second_operand,
                };

                // apply relief after monkey inspection
                item.worry_level /= RELIEF_FACTOR;

                // monkey throw item to another monkey
                let test = match m.test {
                    Action::Div => item.worry_level % m.test_val,
                    _ => panic!("Encountered unexpected test action"),
                };
                let test_result: bool = test == 0;
                if test_result == true {
                    m_vec[m.monkey_true].items.push(item);
                } else {
                    m_vec[m.monkey_false].items.push(item);
                }
            }

            // put the monkey back
            m_vec[m_idx] = m;
        }
    }
}
