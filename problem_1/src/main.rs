use std::io::Read;

fn parse_input(file_name: &str) -> Vec<i32> {
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut v: Vec<i32> = Vec::new();
    for s in contents.lines() {
        v.push(s.parse::<i32>().unwrap());
    }
    v
}

fn find_target_sum(numbers: &[i32], target: i32) -> (i32, i32) {
    let mut result: (i32, i32) = (0, 0);
    let mut counter: usize = 1;
    for val1 in numbers.iter() {
        let other_numbers: &[i32] = numbers.split_at(counter).1;
        counter = counter + 1;
        for val2 in other_numbers.iter() {
            if val1 + val2 == target {
                result.0 = *val1;
                result.1 = *val2;
                break;
            }
        }
    }
    result
}

fn main() {
    let numbers: Vec<i32> = parse_input("input.txt");
    let pair: (i32, i32) = find_target_sum(numbers.as_slice(), 2020);
    if pair.0 != 0 && pair.1 != 0 {
        let product: i32 = pair.0 * pair.1;
        println!("{}", product);
    } else {
        println!("No valid result found.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_target1() {
        assert_eq!(find_target_sum(vec![1, 2].as_slice(), 3), (1, 2));
    }
    
    #[test]
    fn test_find_target2() {
        assert_eq!(find_target_sum(vec![1, 2, 3].as_slice(), 4), (1, 3));
    }

    #[test]
    fn test_find_target3() {
        assert_eq!(find_target_sum(vec![1, 2, 3, 4].as_slice(), 5), (2, 3));
    }

    #[test]
    fn test_find_target4() {
        assert_eq!(find_target_sum(vec![4, 3, 2, 1].as_slice(), 5), (3, 2));
    }

    #[test]
    fn test_find_target5() {
        assert_eq!(find_target_sum(vec![3, 2, 1].as_slice(), 4), (3, 1));
    }

    #[test]
    fn test_find_target6() {
        assert_eq!(find_target_sum(vec![2, 1].as_slice(), 3), (2, 1));
    }
}