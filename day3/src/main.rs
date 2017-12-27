fn manhattan_distance(input: u32) -> u32 {
    if input == 1 {
        return 0;
    }

    // Alternative approach would be to iterate on odd squares and do more
    // math. This solution is still O(width(n)) and easier to read.
    let mut prev;
    let mut current = 1;
    for i in 0.. {
        prev = current;
        current += 1 + (i * 2);
        if input <= current {
            let diff = std::cmp::min(current - input, input - prev);
            return diff + i + 1;
        }
        for _ in 0..3 {
            prev = current;
            current += 2 + (i * 2);
            if input <= current {
                let diff = std::cmp::min(current - input, input - prev);
                return diff + i + 1;
            }
        }
    }
    unreachable!();
}

// This calculates a number which is way larger than necessary. Should optimize.
fn calc_width(n: u32) -> u32 {
    for root in (0..).map(|i| 1 + (2 * i)) {
        if n <= root * root {
            return root + 2;
        }
    }
    unreachable!();
}

// Allocates a 2d array, and fills it in a spiral fashion.
fn stress_test(input: u32) -> u32 {
    if input == 1 {
        return 1;
    };

    let width = calc_width(input) as usize;
    // Gross way to make a 2-d
    let mut grid_raw = vec![0; width * width];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let grid: &mut [&mut [_]] = grid_base.as_mut_slice();
    let mut x = width / 2;
    let mut y = width / 2;
    grid[x][y] = 1;

    for i in 0.. {
        for _ in 0..(1 + (2 * i)) {
            x += 1;
            grid[x][y] = grid[x - 1][y] + grid[x - 1][y + 1] + grid[x][y + 1] + grid[x + 1][y + 1];
            if grid[x][y] > input {
                return grid[x][y];
            }
        }
        for _ in 0..(1 + (2 * i)) {
            y += 1;
            grid[x][y] = grid[x][y - 1] + grid[x - 1][y - 1] + grid[x - 1][y] + grid[x - 1][y + 1];
            if grid[x][y] > input {
                return grid[x][y];
            }
        }
        for _ in 0..(2 + (2 * i)) {
            x -= 1;
            grid[x][y] = grid[x + 1][y] + grid[x + 1][y - 1] + grid[x][y - 1] + grid[x - 1][y - 1];
            if grid[x][y] > input {
                return grid[x][y];
            }
        }
        for _ in 0..(2 + (2 * i)) {
            y -= 1;
            grid[x][y] = grid[x][y + 1] + grid[x + 1][y + 1] + grid[x + 1][y] + grid[x + 1][y - 1];
            if grid[x][y] > input {
                return grid[x][y];
            }
        }
    }
    unreachable!();
}

fn main() {
    let input = 347_991_u32;
    println!("input: {}", input);
    println!("result 1: {}", manhattan_distance(input));
    println!("result 1: {}", stress_test(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(manhattan_distance(1), 0);
        assert_eq!(manhattan_distance(2), 1);
        assert_eq!(manhattan_distance(3), 2);
        assert_eq!(manhattan_distance(12), 3);
        assert_eq!(manhattan_distance(22), 3);
        assert_eq!(manhattan_distance(23), 2);
        assert_eq!(manhattan_distance(1024), 31);
    }

    #[test]
    fn calc_width_test() {
        assert_eq!(calc_width(1), 3);
        assert_eq!(calc_width(2), 5);
        assert_eq!(calc_width(9), 5);
        assert_eq!(calc_width(10), 7);
        assert_eq!(calc_width(17), 7);
    }

    #[test]
    fn stress_test_test() {
        assert_eq!(stress_test(1), 1);
        assert_eq!(stress_test(2), 4);
        assert_eq!(stress_test(3), 4);
        assert_eq!(stress_test(4), 5);
        assert_eq!(stress_test(9), 10);
        assert_eq!(stress_test(22), 23);
        assert_eq!(stress_test(24), 25);
        assert_eq!(stress_test(25), 26);
        assert_eq!(stress_test(53), 54);
    }
}
