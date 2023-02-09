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

use std::{collections::HashMap, io::Write};

const MAX_SIZE: usize = 100000;

#[derive(Debug)]
struct Dir {
    parent: Option<String>,
    files: Vec<File>,
    dirs: Vec<String>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl Dir {
    fn calculate_size(&mut self, dir_sizes: &HashMap<String, usize>) -> usize {
        let mut total = 0;
        for f in &self.files {
            total += f.size;
        }
        for d in &self.dirs {
            total += dir_sizes.get(d).unwrap();
        }

        total
        // Propagate change up directory structure
        // if let Some(parent) = &self.parent {
        //     if let Some(dir_parent) = dirs.get_mut(parent) {
        //         dir_parent.update_size(dirs);
        //     }
        // }
    }

    // fn size(&self) -> usize {
    //     self.size
    // }
}

pub fn solve_p1(input: &str) -> usize {
    let dirs = parse_input(input);
    // sum_dirs(dirs)
    let (size_cumulative, _) = calculate_size(&dirs, &"/".to_string());
    size_cumulative
}

fn parse_input(input: &str) -> HashMap<String, Dir> {
    let mut dirs: HashMap<String, Dir> = HashMap::new();
    let mut dir_current_name: String = String::new();
    // let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    for l in input.lines() {
        let words: Vec<&str> = l.split_ascii_whitespace().collect();
        // println!("{} {:?}", &dir_current_name, words);
        match words.get(1) {
            Some(&"cd") => {
                let dir_name = words.get(2).unwrap();
                if *dir_name == ".." {
                    match &dirs.get(&dir_current_name).unwrap().parent {
                        Some(dir) => {
                            dir_current_name = dir.clone();
                        }
                        None => {}
                    }
                    continue;
                }

                // initialise Dir struct if it doesn't exist,
                // with an exception for root
                let dir = if *dir_name == "/" {
                    Dir {
                        parent: None,
                        files: Vec::new(),
                        dirs: Vec::new(),
                        // size: 0,
                    }
                } else {
                    Dir {
                        // Old current dir is parent
                        parent: Some(dir_current_name.clone()),
                        files: Vec::new(),
                        dirs: Vec::new(),
                        // size: 0,
                    }
                };
                dirs.entry(String::from(*dir_name)).or_insert(dir);
                // dir_sizes.entry(String::from(*dir_name)).or_insert(0);

                match dirs.get_mut(&dir_current_name) {
                    Some(dir) => {
                        dir.dirs.push(dir_name.to_string());
                    }
                    None => {}
                }
                dir_current_name = String::from(*dir_name); // Update to current dir
            }
            Some(&"ls") => {}
            Some(_) => {
                if *words.get(0).unwrap() == "dir" {
                    continue;
                }

                let name = words.get(1).unwrap().to_string();
                let size: usize = words.get(0).unwrap().parse().unwrap();
                let file = File { name, size };

                let dir_current = dirs.get_mut(&dir_current_name).unwrap();
                dir_current.files.push(file);
                // *dir_sizes.get_mut(&dir_current_name).unwrap() = dir_current.calculate_size(&dir_sizes);
                // dir_current.update_size(&dirs);

                // Propagate change up dir tree
                let mut parent = dir_current.parent.clone();
                while let Some(dir_prop_name) = parent {
                    let dir_prop_current = dirs.get_mut(&dir_prop_name).unwrap();
                    // *dir_sizes.get_mut(&dir_prop_name).unwrap() = dir_prop_current.calculate_size(&dir_sizes);
                    parent = dir_prop_current.parent.clone();
                }
            }
            None => {
                panic!("Didn't expect to find nothing!");
            }
        }
    }

    // println!("{:#?}", dirs);
    dirs
}

fn sum_dirs(dir_sizes: HashMap<String, usize>) -> usize {
    let mut total_score = 0;
    for (name, size) in &dir_sizes {
        let size = *size;
        if size <= MAX_SIZE {
            total_score += size;
        }

        println!("{} {}", name, size);
        std::io::stdout().flush().unwrap();
    }

    total_score
}

fn calculate_size(dirs: &HashMap<String, Dir>, dir_current_name: &String) -> (usize, usize) {
    //! Returns (total_cumulative, dir_size)
    let dir_current = dirs.get(dir_current_name).unwrap();
    let mut size_dir = 0;
    let mut size_cumulative = 0;

    for file in &dir_current.files {
        size_dir += file.size;
    }
    for dir_child in &dir_current.dirs {
        let (sc, sd) = calculate_size(dirs, dir_child);
        size_cumulative += sc;
        size_dir += sd;

    }
    if size_dir <= MAX_SIZE {
        size_cumulative += size_dir;
    }

    (size_cumulative, size_dir)
}