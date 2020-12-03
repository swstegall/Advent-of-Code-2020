use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(file_name: &str) -> Vec<String> {
    let br = BufReader::new(File::open(file_name).unwrap());
    let mut v: Vec<String> = Vec::new();
    for line in br.lines() {
        let current = match line {
            Ok(val) => val.clone(),
            Err(_) => panic!("failure to read line.")
        };
        v.push(current);
    }
    v
}

fn count_trees(lines: &Vec<String>) -> i32 {
    0
}

fn main() {
    let lines: Vec<String> = parse_input("input.txt");
    let trees: i32 = count_trees(&lines);
    println!("{}", &trees);
}
