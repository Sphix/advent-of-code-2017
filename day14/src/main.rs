#[macro_use]
extern crate failure;
extern crate itertools;

use failure::Error;
use itertools::Itertools;

use std::ops::Add;

type Ret = (usize, usize);

fn hash_round(list: &mut [u32], seq: &[u8], pos: usize, skip: usize) -> Result<Ret, Error> {
    let mut current_position = pos;
    let mut skip_size = skip;
    let list_len = list.len();

    for num in seq.iter() {
        let num = *num as usize;
        ensure!(num <= list_len, "Invalid number in list.");
        for i in 0..(num / 2) {
            let a = (current_position + i) % list_len;
            let b = (current_position + num - i - 1) % list_len;
            list.swap(a, b);
        }
        current_position += num + skip_size;
        current_position %= list_len;
        skip_size += 1;
    }
    Ok((current_position, skip_size))
}

fn sparse_to_dense_hash(sparse: &[u32]) -> Result<Vec<u32>, Error> {
    const DENSE_LEN: usize = 16;
    ensure!(
        sparse.len() % DENSE_LEN == 0,
        "Sparse hash must be multiple of 16"
    );
    Ok(sparse
        .chunks(DENSE_LEN)
        .map(|chunk| chunk.iter().fold(0, |acc, c| acc ^ c))
        .collect::<Vec<_>>())
}

fn knot_hash(input: &str) -> Result<String, Error> {
    let mut sparse_hash = (0..256).collect::<Vec<u32>>();
    let seq = input
        .bytes()
        .chain(vec![17, 31, 73, 47, 23])
        .collect::<Vec<u8>>();
    let mut pos = 0;
    let mut skip = 0;

    for _ in 0..64 {
        let result = hash_round(&mut sparse_hash, &seq, pos, skip)?;
        pos = result.0;
        skip = result.1;
    }

    let dense_hash = sparse_to_dense_hash(&sparse_hash)?;
    Ok(dense_hash
        .into_iter()
        .map(|n| format!("{:02x}", n))
        .collect::<Vec<String>>()
        .concat())
}

fn count_bits(input: &str) -> u32 {
    const RADIX: u32 = 16;
    input
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap().count_ones())
        .sum()
}

#[derive(PartialEq)]
enum Square {
    Empty,
    Used,
    Region(u32),
}

fn to_vec(input: &str) -> Vec<Square> {
    const RADIX: u32 = 16;
    input
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap())
        .flat_map(|d| vec![(d >> 3) & 1, (d >> 2) & 1, (d >> 1) & 1, d & 1].into_iter())
        .map(|d| if d == 0 { Square::Empty } else { Square::Used })
        .collect::<Vec<Square>>()
}

fn total_bits(input: &str) -> Result<u32, Error> {
    (0..128)
        .map(|i| format!("{}-{}", input, i))
        .map(|k| knot_hash(&k))
        .map_results(|h| count_bits(&h))
        .fold_results(0, Add::add)
}

fn fill_region(grid: &mut Vec<Vec<Square>>, i: usize, j: usize, region: u32) {
    let mut stack = Vec::new();
    stack.push((i, j));

    while let Some((i, j)) = stack.pop() {
        if grid[i][j] != Square::Used {
            continue;
        }
        grid[i][j] = Square::Region(region);
        if i > 0 {
            stack.push((i - 1, j));
        }
        if i < 127 {
            stack.push((i + 1, j));
        }
        if j > 0 {
            stack.push((i, j - 1));
        }
        if j < 127 {
            stack.push((i, j + 1));
        }
    }
}

fn total_regions(input: &str) -> Result<u32, Error> {
    let mut grid: Vec<Vec<Square>> = (0..128)
        .map(|i| format!("{}-{}", input, i))
        .map(|k| knot_hash(&k))
        .map_results(|h| to_vec(&h))
        .collect::<Result<Vec<_>, Error>>()?;

    let mut region = 0;
    (0..128).for_each(|i| {
        (0..128).for_each(|j| {
            if grid[i][j] == Square::Used {
                fill_region(&mut grid, i, j, region);
                region += 1;
            }
        });
    });
    Ok(region)
}

fn main() {
    let input = "hxtvlmkl";

    let result = total_bits(&input).expect("parse");
    println!("Result 1: {}", result);

    let result = total_regions(&input).expect("parse");
    println!("Result 2: {}", result);
}
