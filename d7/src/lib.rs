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

use std::{collections::HashMap};

const MAX_SIZE: usize = 100000;
const FILESYSTEM_SIZE: usize = 70000000;
const UNUSED_SPACE_NEEDED: usize = 30000000;

#[derive(Default, Debug)]
struct GlobalData {
    root: Dir,
}

#[derive(Default, Debug)]
struct Dir {
    dirs: HashMap<String, Dir>,
    files: HashMap<String, usize>,
}

pub fn solve(input: &str) -> (usize, usize) {
    //! Returns (part_1, part_2)
    let mut data = GlobalData::default();
    data.parse_input(input);
    let (size_cumulative, size, mut sizes) = data.calculate_size(&data.root, Vec::new());
    sizes.push(size);
    let min_size = find_smallest_size(sizes);

    (size_cumulative, min_size)
}

impl GlobalData {
    fn parse_input(&mut self, input: &str) {
        let mut path: Vec<String> = Vec::new();

        for line in input.lines() {
            let line = line.trim();
            if line.chars().nth(0).unwrap() == '$' {
                if line.chars().nth(2).unwrap() == 'c' {
                    let dir = &line[5..];
                    if dir.chars().nth(0).unwrap() == '/' {
                        path = Vec::new();
                    } else if dir == ".." {
                        path.pop().unwrap();
                    } else {
                        let cwd = self.get_dir_mut(&path);
                        if cwd.dirs.contains_key(dir) == false {
                            cwd.dirs.insert(dir.to_string(), Dir::default());
                        }
                        path.push(dir.to_string());
                    }
                }
            } else {
                let words: Vec<&str> = line.split_whitespace().collect();
                if *words.get(0).unwrap() == "dir" {
                    let dir = words.get(1).unwrap();
                    let cwd = self.get_dir_mut(&path);
                    if cwd.dirs.contains_key(*dir) == false {
                        cwd.dirs.insert(dir.to_string(), Dir::default());
                    }
                } else {
                    let cwd = self.get_dir_mut(&path);
                    cwd.files.insert(
                        words.get(1).unwrap().to_string(),
                        words.get(0).unwrap().parse().unwrap(),
                    );
                }
            }
        }
    }

    fn get_dir_mut(&mut self, path: &Vec<String>) -> &mut Dir {
        let mut dir_current = &mut self.root;
        for dir_name in path {
            dir_current = dir_current.dirs.get_mut(dir_name).unwrap();
        }

        dir_current
    }

    fn calculate_size(&self, dir_current: &Dir, mut sizes: Vec<usize>) -> (usize, usize, Vec<usize>) {
        //! Returns (total_cumulative, dir_size)
        let mut size_dir = 0;
        let mut size_cumulative = 0;

        for file in &dir_current.files {
            size_dir += file.1;
        }
        for dir_child in &dir_current.dirs {
            let (sc, sd, sv) = self.calculate_size(dir_child.1, sizes);
            sizes = sv;
            size_cumulative += sc;
            size_dir += sd;
            sizes.push(sd);
        }
        if size_dir <= MAX_SIZE {
            size_cumulative += size_dir;
        }

        (size_cumulative, size_dir, sizes.clone())
    }
}

fn find_smallest_size(sizes: Vec<usize>) -> usize {
    let mut min_dir_size: usize = usize::MAX;
    let max_size = sizes.iter().max().unwrap();
    let size_target = UNUSED_SPACE_NEEDED - (FILESYSTEM_SIZE - max_size);
    for size in sizes {
        if let Some(result) = size.checked_sub(size_target) {
            if result < min_dir_size - size_target {
                min_dir_size = size;
            }
        }
    }

    min_dir_size
}