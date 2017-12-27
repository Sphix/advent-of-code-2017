#[macro_use]
extern crate failure;

use failure::Error;

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

fn one_round_hash(input: &str, array_len: u32) -> Result<u32, Error> {
    let mut list = (0..array_len).collect::<Vec<u32>>();
    let input = input
        .split(',')
        .filter_map(|n| str::parse::<u8>(n).ok())
        .collect::<Vec<_>>();
    hash_round(&mut list, &input, 0, 0)?;
    Ok(list[0] * list[1])
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

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();

    println!(
        "Result 1: {}",
        one_round_hash(input, 256).expect("don't fail.")
    );
    println!("Result 2: {}", knot_hash(input).expect("don't fail."));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_round_hash_test() {
        assert_eq!(one_round_hash("3,4,1,5", 5).unwrap(), 12);
    }

    #[test]
    fn knot_hash_test() {
        assert_eq!(knot_hash("").unwrap(), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(
            knot_hash("AoC 2017").unwrap(),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(
            knot_hash("1,2,3").unwrap(),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        assert_eq!(
            knot_hash("1,2,4").unwrap(),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
