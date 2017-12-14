extern crate bit_tools;
extern crate knot;

use knot::KnotHash;
use bit_tools::{AsBits, Bit};

fn main() {
    let puzzle_input = "wenycdww";

    let hashes = construct_hashes(&puzzle_input);

    let squares_used = count_ones(&hashes);

    println!("{} squares are used.", squares_used);
}

fn construct_hashes(seed: &str) -> Vec<KnotHash> {
    let mut hashes: Vec<KnotHash> = Vec::with_capacity(128);
    for n in 0..128 {
        let hash_input = format!("{}-{}", seed, n);
        hashes.push(KnotHash::new(&hash_input));
    }

    hashes
}

fn count_ones(hashes: &[KnotHash]) -> usize {
    hashes
        .iter()
        .map(|hash| hash.as_bytes().as_bits())
        .map(|bits| bits.iter().filter(|&bit| *bit == Bit::One).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let hashes = construct_hashes("flqrgnkx");
        assert_eq!(count_ones(&hashes), 8108);
    }
}
