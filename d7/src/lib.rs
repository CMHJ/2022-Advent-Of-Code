/*!
--- Day 7: No Space Left On Device ---
Input is a series of Unix like inputs.
Goal is to parse this input and determine directories to delete to free up
system space.

Need to determine size of directories which are the sum of the file sizes in
them.  Directories have no size themselves.

Find directories with a total size of at most 100000, then calculate the sum of
their sizes.  Directories within other directories are counted, meaning that
files are counted more than once, it's the size of the directory that counts.
 */

#[cfg(test)]
mod test;

use std::collections::HashMap;

const MAX_SIZE: usize = 100000;

#[derive(Default, Debug)]
struct GlobalData {
    root: Dir,
}

#[derive(Default, Debug)]
struct Dir {
    dirs: HashMap<String, Dir>,
    files: HashMap<String, usize>,
}

pub fn solve_p1(input: &str) -> usize {
    let mut data = GlobalData::default();
    data.parse_input(input);
    // println!("{:#?}", data);
    let (size_cumulative, _) = data.calculate_size(&data.root);
    size_cumulative
}

impl GlobalData {
    fn parse_input(&mut self, input: &str) {
        let mut stack: Vec<String> = Vec::new();

        for line in input.lines() {
            let line = line.trim();
            if line.chars().nth(0).unwrap() == '$' {
                if line.chars().nth(2).unwrap() == 'c' {
                    let dir = &line[5..];
                    if dir.chars().nth(0).unwrap() == '/' {
                        stack = Vec::new();
                    } else if dir == ".." {
                        stack.pop().unwrap();
                    } else {
                        let cwd = self.get_dir_mut(&stack);
                        if cwd.dirs.contains_key(dir) == false {
                            cwd.dirs.insert(dir.to_string(), Dir::default());
                        }
                        stack.push(dir.to_string());
                    }
                }
            } else {
                let words: Vec<&str> = line.split_whitespace().collect();
                if *words.get(0).unwrap() == "dir" {
                    let dir = words.get(1).unwrap();
                    let cwd = self.get_dir_mut(&stack);
                    if cwd.dirs.contains_key(*dir) == false {
                        cwd.dirs.insert(dir.to_string(), Dir::default());
                    }
                } else {
                    let cwd = self.get_dir_mut(&stack);
                    cwd.files.insert(
                        words.get(1).unwrap().to_string(),
                        words.get(0).unwrap().parse().unwrap(),
                    );
                }
            }
        }
    }

    fn get_dir_mut(&mut self, stack: &Vec<String>) -> &mut Dir {
        let mut dir_current = &mut self.root;
        for dir_name in stack {
            dir_current = dir_current.dirs.get_mut(dir_name).unwrap();
        }

        dir_current
    }

    fn calculate_size(&self, dir_current: &Dir) -> (usize, usize) {
        //! Returns (total_cumulative, dir_size)
        let mut size_dir = 0;
        let mut size_cumulative = 0;

        for file in &dir_current.files {
            size_dir += file.1;
        }
        for dir_child in &dir_current.dirs {
            let (sc, sd) = self.calculate_size(dir_child.1);
            size_cumulative += sc;
            size_dir += sd;
        }
        if size_dir <= MAX_SIZE {
            size_cumulative += size_dir;
        }

        (size_cumulative, size_dir)
    }
}
