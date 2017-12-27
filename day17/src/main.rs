use std::collections::VecDeque;

fn find_value(steps: usize, iterations: usize, value: usize) -> usize {
    let mut spinlock = VecDeque::with_capacity(iterations);
    spinlock.push_back(0);
    (1..iterations)
        .scan(0, |prev, i| {
            *prev = ((*prev + steps) % i) + 1;
            Some((*prev, i))
        })
        .for_each(|(pos, i)| spinlock.insert(pos, i));
    spinlock[(spinlock.iter().position(|&n| n == value).unwrap() + 1) % spinlock.len()]
}

fn find_zero(steps: usize, iterations: usize) -> usize {
    let mut zero_idx = 0;
    let mut expected_value = 0;
    (1..iterations)
        .scan(0, |prev, i| {
            *prev = ((*prev + steps) % i) + 1;
            Some((*prev, i))
        })
        .for_each(|(pos, i)| {
            if pos - 1 < zero_idx {
                zero_idx += 1;
            } else if pos - 1 == zero_idx {
                expected_value = i;
            }
        });
    expected_value
}

fn main() {
    let val = create_spin_lock(386, 2018, 2017);
    println!("Result 1: {}", val);

    let val = find_zero(386, 50_000_000);
    println!("Result 2: {}", val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_spin_lock_test() {
        assert_eq!(create_spin_lock(3, 2017), 638);
    }
}
