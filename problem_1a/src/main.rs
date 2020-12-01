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

fn find_target_sum(numbers: &[i32], target: i32) -> (i32, i32, i32) {
    let outer_numbers: &[i32] = numbers;
    let mut result: (i32, i32, i32) = (0, 0, 0);
    let mut outer_counter: usize = 1;
    for val1 in outer_numbers.iter() {
        let middle_numbers: &[i32] = outer_numbers.split_at(outer_counter).1;
        outer_counter = outer_counter + 1;
        let mut middle_counter = 1;
        for val2 in middle_numbers.iter() {
            let inner_numbers: &[i32] = middle_numbers.split_at(middle_counter).1;
            middle_counter = middle_counter + 1;
            for val3 in inner_numbers.iter() {
                if val1 + val2 + val3 == target {
                    result.0 = *val1;
                    result.1 = *val2;
                    result.2 = *val3;
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    let numbers: Vec<i32> = parse_input("input.txt");
    let triad: (i32, i32, i32) = find_target_sum(numbers.as_slice(), 2020);
    if triad.0 != 0 && triad.1 != 0 && triad.2 != 0 {
        let product: i32 = triad.0 * triad.1 * triad.2;
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
        assert_eq!(find_target_sum(vec![1, 2, 3, 4].as_slice(), 6), (1, 2, 3));
    }

    #[test]
    fn test_find_target2() {
        assert_eq!(find_target_sum(vec![1, 2, 3, 4].as_slice(), 7), (1, 2, 4));
    }

    #[test]
    fn test_find_target3() {
        assert_eq!(find_target_sum(vec![1, 2, 3, 4].as_slice(), 8), (1, 3, 4));
    }

    #[test]
    fn test_find_target4() {
        assert_eq!(find_target_sum(vec![1, 2, 3, 4].as_slice(), 9), (2, 3, 4));
    }

    #[test]
    fn test_find_target5() {
        assert_eq!(find_target_sum(vec![4, 3, 2, 1].as_slice(), 6), (3, 2, 1));
    }

    #[test]
    fn test_find_target6() {
        assert_eq!(find_target_sum(vec![4, 3, 2, 1].as_slice(), 7), (4, 2, 1));
    }

    #[test]
    fn test_find_target7() {
        assert_eq!(find_target_sum(vec![4, 3, 2, 1].as_slice(), 8), (4, 3, 1));
    }

    #[test]
    fn test_find_target8() {
        assert_eq!(find_target_sum(vec![4, 3, 2, 1].as_slice(), 9), (4, 3, 2));
    }
}