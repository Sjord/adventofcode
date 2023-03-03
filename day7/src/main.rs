use std::{env, fs, path::PathBuf};

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let lines = parse_dir(&contents);
    let files = parse_lines(lines);
    println!("{:?}", files);
}

fn parse_dir(contents: &str) -> Vec<Line> {
    contents.lines().map(|l| {
        if l.starts_with("$ cd") {
            let parts : Vec<_> = l.split_ascii_whitespace().collect();
            let dir = parts[2];
            Line::Command(Command::Dir(dir.to_owned()))
        } else  if l.starts_with("$ ls") {
            Line::Command(Command::Ls)
        } else {
            let parts : Vec<_> = l.split_ascii_whitespace().collect();
            if parts[0] == "dir" {
                let name = parts[1];
                Line::Inode(Inode::Dir(Dir { name: name.to_owned() }))

            } else {
                let size: usize = parts[0].parse().unwrap();
                let name = parts[1];
                Line::Inode(Inode::File(File { size, name: name.to_owned() }))
            }
        }
    }).collect()
}

fn parse_lines(lines: Vec<Line>) -> Vec<PFile> {
    let mut result = Vec::new();
    let mut current_path = PathBuf::new();
    for l in lines {
        match l {
            Line::Command(Command::Dir(name)) => {
                match name.as_str() {
                    ".." => { current_path.pop(); },
                    _ => current_path.push(name),
                }
            }
            Line::Command(Command::Ls) => (),
            Line::Inode(Inode::File(f)) => {
                result.push(PFile {
                    size: f.size,
                    path: current_path.join(f.name)
                })
            },
            Line::Inode(Inode::Dir(d)) => (),
        }
        // println!("{:?}", currentPath);
    }
    result
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Inode(Inode),
}

#[derive(Debug)]
enum Command {
    Ls,
    Dir(String)
}

#[derive(Debug)]
enum Inode {
    File(File),
    Dir(Dir),
}    

#[derive(Debug)]
struct File {
    size: usize,
    name: String
}

#[derive(Debug)]
struct Dir {
    name: String
}

#[derive(Debug)]
struct PFile {
    size: usize,
    path: PathBuf
}
