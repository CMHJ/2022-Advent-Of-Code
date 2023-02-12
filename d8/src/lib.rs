#![allow(dead_code)]
/*!
--- Day 8: Treetop Tree House ---
Need to determine if there is enough tree cover to keep tree house hidden.
This is done by counting the number of trees that are visible from outside the
grid when looking directly along a row or column.
 */

#[cfg(test)]
mod test;

type TreeMap = Vec<Vec<Tree>>;

#[derive(Debug, Default)]
struct Tree {
    height: i8,
    visible: bool,
}

pub fn solve_p1(input: &str) -> usize {
    let mut tree_map = parse_input(input);
    mark_visible_trees(&mut tree_map);
    // print_tree_map_height(&tree_map);
    // println!();
    // print_tree_map_visible(&tree_map);
    let n = count_visible_trees(&tree_map);
    n
}

fn parse_input(input: &str) -> TreeMap {
    let mut tree_map = TreeMap::new();
    // Parse numbers into 2D vector of u8s
    for line in input.lines() {
        let mut tree_row: Vec<Tree> = Vec::with_capacity(line.len());
        for c in line.chars() {
            let tree = Tree {
                height: c.to_digit(10).expect("Could not convert to number") as i8,
                visible: false,
            };
            tree_row.push(tree);
        }

        tree_map.push(tree_row);
    }
    // Create matching 2D vector of bools to mark trees that can be seen

    tree_map
}

fn mark_visible_trees(tree_map: &mut TreeMap) {
    let length_row = tree_map.get(0).unwrap().len();
    let length_column = tree_map.len();

    // Iterate through rows
    for row in tree_map.iter_mut() {
        let mut height_max: i8 = i8::MIN;
        for tree in row {
            if tree.height > height_max {
                tree.visible = true;
                height_max = tree.height;
            }
        }
    }

    // Iterate through rows in reverse
    for row in tree_map.iter_mut() {
        let mut height_max: i8 = i8::MIN;
        for tree in row.iter_mut().rev() {
            if tree.height > height_max {
                tree.visible = true;
                height_max = tree.height;
            }
        }
    }

    // Iterate through columns
    for c in 0..length_row {
        let mut height_max: i8 = i8::MIN;
        for r in 0..length_column {
            let mut tree = tree_map.get_mut(r).unwrap().get_mut(c).unwrap();
            if tree.height > height_max {
                tree.visible = true;
                height_max = tree.height;
            }
        }
    }

    // Iterate through columns in reverse
    for c in 0..length_row {
        let mut height_max: i8 = i8::MIN;
        for r in (0..length_column).rev() {
            let mut tree = tree_map.get_mut(r).unwrap().get_mut(c).unwrap();
            if tree.height > height_max {
                tree.visible = true;
                height_max = tree.height;
            }
        }
    }
}

fn count_visible_trees(tree_map: &TreeMap) -> usize {
    let mut c = 0;
    for row in tree_map {
        for tree in row {
            if tree.visible {
                c += 1;
            }
        }
    }

    c
}

fn print_tree_map_visible(tree_map: &TreeMap) {
    for row in tree_map {
        for tree in row {
            if tree.visible {
                print!("X");
            } else {
                print!("O");
            }
        }
        println!();
    }
}

fn print_tree_map_height(tree_map: &TreeMap) {
    for row in tree_map {
        for tree in row {
            print!("{}", tree.height);
        }
        println!();
    }
}
