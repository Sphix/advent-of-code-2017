fn compute_checksum(input: &str) -> i32 {
    input.split('\n').map(|line| {
        let (max, min) = line.split_whitespace()
            .map(|n| str::parse::<i32>(n).unwrap())
            .fold((std::i32::MIN, std::i32::MAX), |(max, min), num| 
                (std::cmp::max(num, max), std::cmp::min(num, min))
            );
        max - min
    }).sum()
}

fn find_divisible_numbers(numbers: Vec<i32>) -> Option<(i32, i32)> {
    let len = numbers.len();
    for i in 0..len {
        for j in 0..len {
            if i == j {
                continue;
            }
            if numbers[i] % numbers[j] == 0 {
                return Some((numbers[i], numbers[j]));
            }
        }
    }
    None
}

fn compute_checksum_2(input: &str) -> i32 {
    input.split('\n').map(|line| {
        let numbers = line.split_whitespace()
            .map(|n| str::parse::<i32>(n))
            .collect::<Result<Vec<_>, _>>().expect("parse");
        if let Some((first, second)) = find_divisible_numbers(numbers) {
            first / second
        } else {
            panic!("No divisible numbers found in {}", line);
        }
    }).sum()
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();
    println!("input:\n{}\n", input);
    println!("result 1: {}", compute_checksum(input));
    println!("result 2: {}", compute_checksum_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_checksum_test() {
        assert_eq!(compute_checksum("5 1 9 5\n7 5 3\n2 4 6 8"), 18);
    }

    #[test]
    fn compute_checksum_2_test() {
        assert_eq!(compute_checksum_2("5 9 2 8\n9 4 7 3\n3 8 6 5"), 9);
    }
}
