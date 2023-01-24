use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::{
    cmp::min,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type FileTree = HashMap<String, FileEntry>;

enum FileEntry {
    File { size: usize },
    Directory { files: FileTree },
}

impl FileEntry {
    fn get_size(&self) -> usize {
        match self {
            FileEntry::File { size } => *size,
            FileEntry::Directory { files } => {
                files.iter().fold(0, |acc, (_, e)| acc + e.get_size())
            }
        }
    }
}

fn insert_file(
    tree: &mut FileTree,
    relative_path: &[String],
    filename: String,
    file: FileEntry,
) -> Result<()> {
    if relative_path.is_empty() {
        tree.insert(filename, file);
        return Ok(());
    }

    let dirname = relative_path[0].to_string();
    let remaining_path = &relative_path[1..];

    let dir_entry = tree.entry(dirname).or_insert_with(|| FileEntry::Directory {
        files: FileTree::new(),
    });
    let subtree = if let FileEntry::Directory { files } = dir_entry {
        files
    } else {
        bail!("is not a directory")
    };

    insert_file(subtree, remaining_path, filename, file)
}

fn sum_dir_size(tree: &FileTree) -> usize {
    tree.iter().fold(0, |acc, (_, e)| {
        acc + match e {
            FileEntry::Directory { files } => {
                let dir_size = e.get_size();
                if dir_size <= 100_000 {
                    dir_size + sum_dir_size(files)
                } else {
                    sum_dir_size(files)
                }
            }
            _ => 0,
        }
    })
}

fn find_smallest_dir_size(tree: &FileTree, min_size: usize) -> usize {
    tree.iter().fold(std::usize::MAX, |best, (_, e)| match e {
        FileEntry::Directory { files } => {
            let dir_size = e.get_size();
            if dir_size >= min_size {
                min(min(best, dir_size), find_smallest_dir_size(files, min_size))
            } else {
                min(best, find_smallest_dir_size(files, min_size))
            }
        }
        _ => best,
    })
}

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/07.txt")?;
    let mut root: FileTree = HashMap::new();
    let mut curr_path: Vec<String> = Vec::new();
    let mut lines = BufReader::new(file).lines();
    while let Some(line) = lines.next() {
        let line = line?;
        let tokens = line.split(' ').collect_vec();
        match tokens[0] {
            "$" => {
                match tokens[1] {
                    "cd" => {
                        match tokens[2] {
                            "/" => curr_path.clear(),
                            ".." => {
                                curr_path.pop();
                            }
                            dirname => {
                                curr_path.push(dirname.to_string());
                            }
                        };
                    }
                    "ls" => {}
                    s => bail!("Unknown command: {}", s),
                };
            }
            // ls output
            "dir" => {}
            str_size => {
                let size = str_size
                    .parse::<usize>()
                    .with_context(|| format!("is not a file size: {}", str_size))?;
                let filename = tokens[1].to_string();
                insert_file(&mut root, &curr_path, filename, FileEntry::File { size })?;
            }
        }
    }
    let total_size = sum_dir_size(&root);
    println!("{total_size}");

    // part 2
    let remaining_space = 70_000_000 - root.iter().fold(0, |acc, (_, e)| acc + e.get_size());
    let required_space = 30_000_000;
    let space_to_free = required_space - remaining_space;

    let dir_size_to_free = find_smallest_dir_size(&root, space_to_free);
    println!("{dir_size_to_free}");

    Ok(())
}
