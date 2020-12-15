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

fn main() -> Result<(), std::io::Error> {
    let lines: Vec<String> = parse_input("input.txt");
    println!("{:?}", lines);
    Ok(())
}

