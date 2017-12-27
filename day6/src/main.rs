use std::collections::{HashMap, HashSet};

fn largest_block_index(blocks: &Vec<u32>) -> (usize, u32) {
    let mut index = 0;
    let mut max = blocks[0];
    for i in 1..blocks.len() {
        if blocks[i] > max {
            index = i;
            max = blocks[i];
        }
    }
    (index, max)
}

fn reallocate_blocks(blocks: &mut Vec<u32>) -> u32 {
    let len = blocks.len();
    let mut steps = 0;
    let mut set = HashSet::new();
    while !set.contains(blocks) {
        set.insert(blocks.clone());
        let (mut position, amount) = largest_block_index(blocks);
        blocks[position] = 0;
        for _ in 0..amount {
            position = (position + 1) % len;
            blocks[position] += 1;
        }
        steps += 1;
    }
    steps
}

fn reallocate_blocks_2(blocks: &mut Vec<u32>) -> u32 {
    let len = blocks.len();
    let mut current_step = 0u32;
    let mut map = HashMap::new();
    while !map.contains_key(blocks) {
        map.insert(blocks.clone(), current_step);
        let (mut position, amount) = largest_block_index(blocks);
        blocks[position] = 0;
        for _ in 0..amount {
            position = (position + 1) % len;
            blocks[position] += 1;
        }
        current_step += 1;
    }
    current_step - map.get(blocks).unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let mut input = input
        .split_whitespace()
        .filter_map(|n| str::parse::<u32>(n).ok())
        .collect::<Vec<_>>();
    println!("input:\n{:?}\n", input);
    println!("result 1: {}", reallocate_blocks(&mut input.clone()));
    println!("result 2: {}", reallocate_blocks_2(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_block_index_test() {
        assert_eq!(largest_block_index(&vec![1]), (0, 1));
        assert_eq!(largest_block_index(&vec![0, 2, 7, 0]), (2, 7));
    }

    #[test]
    fn reallocate_blocks_test() {
        assert_eq!(reallocate_blocks(&mut vec![0, 2, 7, 0]), 5);
    }

    #[test]
    fn reallocate_blocks_2_test() {
        assert_eq!(reallocate_blocks_2(&mut vec![0, 2, 7, 0]), 4);
    }
}
