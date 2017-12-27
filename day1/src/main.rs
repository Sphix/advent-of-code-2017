fn compute_captcha(input: &str) -> u32 {
    const RADIX: u32 = 10;
    let chars = input.chars().collect::<Vec<_>>();
    let len = chars.len();

    let mut count = 0;
    for i in 0..len {
        if chars[i] == chars[(i + 1) % len] {
            count += chars[i].to_digit(RADIX).unwrap();
        }
    }
    count
}

fn compute_captcha_2(input: &str) -> u32 {
    const RADIX: u32 = 10;
    let chars = input.chars().collect::<Vec<_>>();
    let len = chars.len();

    let mut count = 0;
    for i in 0..len {
        if chars[i] == chars[(i + (len / 2)) % len] {
            count += chars[i].to_digit(RADIX).unwrap();
        }
    }
    count
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();
    println!("input:\n{}\n", input);
    println!("result 1: {}", compute_captcha(input));
    println!("result 2: {}", compute_captcha_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_captcha_test() {
        assert_eq!(compute_captcha("1122"), 3);
        assert_eq!(compute_captcha("1111"), 4);
        assert_eq!(compute_captcha("1234"), 0);
        assert_eq!(compute_captcha("91212129"), 9);
    }

    #[test]
    fn compute_captcha_2_test() {
        assert_eq!(compute_captcha_2("1212"), 6);
        assert_eq!(compute_captcha_2("1221"), 0);
        assert_eq!(compute_captcha_2("123123"), 12);
        assert_eq!(compute_captcha_2("12131415"), 4);
    }
}
