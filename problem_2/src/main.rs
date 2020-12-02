use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(file_name: &str) -> Vec<(i32, i32, String, String)> {
    let br = BufReader::new(File::open(file_name).unwrap());
    let mut v: Vec<(i32, i32, String, String)> = Vec::new();
    for line in br.lines() {
        let current = match line {
            Ok(val) => val.clone(),
            Err(_) => panic!("failure to read line.")
        };
        let mut current = current
            .trim()
            .split_whitespace();
        let nums: Vec<&str> = current
            .next()
            .unwrap()
            .split('-')
            .collect();
        let candidate = String::from(String::from(current
            .next()
            .unwrap())
            .get(0..1)
            .unwrap());
        let password = String::from(current
            .next()
            .unwrap());
        v.push((nums[0].parse::<i32>().unwrap(), nums[1].parse::<i32>().unwrap(), candidate, password));
    }
    v
}

fn check_valid_passwords(passwords: Vec<(i32, i32, String, String)>, part1: bool) -> i32 {
    let mut counter = 0;
    for password in passwords {
        if check_valid_password(password, part1) {
            counter = counter + 1;
        }
    }
    return counter;
}

fn check_valid_password(password: (i32, i32, String, String), part1: bool) -> bool {
    let first = password.0;
    let second = password.1;
    let mut counter = 0;
    let test_char = password.2;
    let unverified_password = password.3.as_bytes();
    if part1 {
        for character in unverified_password.iter().enumerate() {
            if character.1 == &test_char.as_bytes()[0] {
                counter = counter + 1;
            }
        }
        if counter >= first && counter <= second {
            return true;
        }
    } else {
        let mut occurrences_counter = 0;
        if unverified_password[(first - 1) as usize] == test_char.as_bytes()[0] {
            occurrences_counter = occurrences_counter + 1;
        }
        if unverified_password[(second - 1) as usize] == test_char.as_bytes()[0] {
            occurrences_counter = occurrences_counter + 1;
        }
        if occurrences_counter == 1 {
            return true;
        }
    }
    false
}

fn main() {
    let lines = parse_input("input.txt");
    let valid_passwords = check_valid_passwords(lines, false);
    println!("{:?}", valid_passwords);
}
