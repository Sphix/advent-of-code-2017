struct Generator {
    curr: u64,
    factor: u64,
    multiple: u64,
}

impl Generator {
    fn new(start: u64, factor: u64, multiple: u64) -> Generator {
        Generator {
            curr: start,
            factor: factor,
            multiple: multiple,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.curr = self.curr * self.factor % 2147483647;
            if self.curr % self.multiple == 0 {
                return Some(self.curr);
            }
        }
    }
}

fn judge_generators(a: Generator, b: Generator, iterations: usize) -> u64 {
    a.zip(b)
        .take(iterations)
        .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count() as u64
}

fn main() {
    let result = judge_generators(
        Generator::new(873, 16807, 1),
        Generator::new(583, 48271, 1),
        40_000_000,
    );
    println!("result 1: {}", result);

    let result = judge_generators(
        Generator::new(873, 16807, 4),
        Generator::new(583, 48271, 8),
        5_000_000,
    );
    println!("result 2: {}", result);
}

#[test]
fn judge_generators_test() {
    let result = judge_generators(
        Generator::new(65, 16807, 1),
        Generator::new(8921, 48271, 1),
        40_000_000,
    );
    assert_eq!(result, 588);
}

#[test]
fn judge_generators_2_test() {
    let result = judge_generators(
        Generator::new(65, 16807, 4),
        Generator::new(8921, 48271, 8),
        5_000_000,
    );
    assert_eq!(result, 309);
}
