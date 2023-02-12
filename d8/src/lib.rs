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
    scenic_score: usize,
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

pub fn solve_p2(input: &str) -> usize {
    let tree_map = parse_input(input);
    let max_score = calculate_scenic_scores(&tree_map);
    max_score
}

fn parse_input(input: &str) -> TreeMap {
    let mut tree_map = TreeMap::new();
    for line in input.lines() {
        let mut tree_row: Vec<Tree> = Vec::with_capacity(line.len());
        for c in line.chars() {
            let tree = Tree {
                height: c.to_digit(10).expect("Could not convert to number") as i8,
                visible: false,
                scenic_score: 0,
            };

            tree_row.push(tree);
        }

        tree_map.push(tree_row);
    }

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

fn calculate_scenic_scores(tree_map: &TreeMap) -> usize {
    let length_row = tree_map.get(0).unwrap().len();
    let length_column = tree_map.len();
    let mut max_score: usize = 0;

    for r in 1..(length_column - 1) {
        for c in 1..(length_row - 1) {
            let current_scenic_score = calculate_scenic_score(tree_map, r, c);

            if current_scenic_score > max_score {
                max_score = current_scenic_score;
            }
        }
    }

    max_score
}

fn calculate_scenic_score(tree_map: &TreeMap, r: usize, c: usize) -> usize {
    let length_row = tree_map.get(0).unwrap().len();
    let length_column = tree_map.len();
    let tree = tree_map.get(r).unwrap().get(c).unwrap();
    let mut current_scenic_score: usize = 1;

    // Look up
    let mut view_score: usize = 0;
    for i in (0..r).rev() {
        let tree_in_view = tree_map.get(i).unwrap().get(c).unwrap();
        view_score += 1;
        if tree_in_view.height >= tree.height {
            break;
        }
    }
    current_scenic_score *= view_score;

    // Look left
    let mut view_score: usize = 0;
    for i in (0..c).rev() {
        let tree_in_view = tree_map.get(r).unwrap().get(i).unwrap();
        view_score += 1;
        if tree_in_view.height >= tree.height {
            break;
        }
    }
    current_scenic_score *= view_score;

    // Look right
    let mut view_score: usize = 0;
    for i in (c + 1)..length_row {
        let tree_in_view = tree_map.get(r).unwrap().get(i).unwrap();
        view_score += 1;
        if tree_in_view.height >= tree.height {
            break;
        }
    }
    current_scenic_score *= view_score;

    // Look down
    let mut view_score: usize = 0;
    for i in (r + 1)..length_column {
        let tree_in_view = tree_map.get(i).unwrap().get(c).unwrap();
        view_score += 1;
        if tree_in_view.height >= tree.height {
            break;
        }
    }
    current_scenic_score *= view_score;

    current_scenic_score
}
