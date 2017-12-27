fn escape_maze(jump_list: &mut Vec<i32>) -> u32 {
    let mut steps = 0;
    let mut position: i32 = 0;
    while (position as usize) < jump_list.len() {
        let jump_offset = jump_list[position as usize];
        jump_list[position as usize] += 1;
        position += jump_offset;
        steps += 1;
    }
    steps
}

fn escape_maze_2(jump_list: &mut Vec<i32>) -> u32 {
    let mut steps = 0;
    let mut position: i32 = 0;
    while (position as usize) < jump_list.len() {
        let jump_offset = jump_list[position as usize];
        if jump_offset >= 3 {
            jump_list[position as usize] -= 1;
        } else {
            jump_list[position as usize] += 1;
        }
        position += jump_offset;
        steps += 1;
    }
    steps
}

fn main() {
    let input = include_str!("input.txt");
    let mut input = input
        .split_whitespace()
        .filter_map(|n| str::parse::<i32>(n).ok())
        .collect::<Vec<_>>();
    println!("input:\n{:?}\n", input);
    println!("result 1: {}", escape_maze(&mut input.clone()));
    println!("result 2: {}", escape_maze_2(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_maze_test() {
        assert_eq!(escape_maze(&mut vec![0, 3, 0, 1, -3]), 5);
    }

    #[test]
    fn escape_maze_2_test() {
        assert_eq!(escape_maze_2(&mut vec![0, 3, 0, 1, -3]), 10);
    }
}
