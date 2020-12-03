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

fn count_trees(lines: &Vec<String>, x_jump_size: usize, y_jump_size: usize) -> u32 {
    let width = lines[0].len();
    lines.iter()
        .enumerate()
        .filter(|(i, line)| {
            let mut chars = line.chars();
            (i % y_jump_size == 0) && (chars.nth(x_jump_size * i / y_jump_size % width).unwrap() == '#')
        })
        .count() as u32
}

fn main() -> Result<(), std::io::Error> {
    let lines: Vec<String> = parse_input("input.txt");
    let trees: u32 = count_trees(&lines, 1, 1);
    let trees1: u32 = count_trees(&lines, 3, 1);
    let trees2: u32 = count_trees(&lines, 5, 1);
    let trees3: u32 = count_trees(&lines, 7, 1);
    let trees4: u32 = count_trees(&lines, 1, 2);
    let result: u32 = &trees * &trees1 * &trees2 * &trees3 * &trees4;
    println!("{}", result);
    Ok(())
}
