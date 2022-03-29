use std::{env, fs};

fn main() {
    walk_dir();
}

fn walk_dir() {
    let current_dir = env::current_dir().unwrap();
    println!("current_dir: {}", current_dir.display());
    for entry in fs::read_dir(current_dir).unwrap() {
        let path = entry.unwrap().path();
        println!("{}", path.display())
    }
}
