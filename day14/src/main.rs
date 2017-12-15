#[macro_use]
extern crate lazy_static;

extern crate bit_tools;
extern crate knot;

use std::collections::HashSet;
use knot::KnotHash;
use bit_tools::{AsBits, Bit};

fn main() {
    let puzzle_input = "wenycdww";

    let squares = construct_hashes(puzzle_input);

    let squares_used = count_ones(&squares);

    println!("{} squares are used.", squares_used);

    let regions = count_regions(&squares);

    println!("There are {} regions.", regions);
}

fn construct_hashes(seed: &str) -> Vec<Vec<Bit>> {
    let mut hashes: Vec<Vec<Bit>> = Vec::with_capacity(128);
    for n in 0..128 {
        let hash_input = format!("{}-{}", seed, n);
        hashes.push(KnotHash::new(&hash_input).as_bytes().as_bits());
    }

    hashes
}

fn count_ones(squares: &[Vec<Bit>]) -> usize {
    squares
        .iter()
        .map(|bits| bits.iter().filter(|&bit| *bit == Bit::One).count())
        .sum()
}

fn count_regions(squares: &[Vec<Bit>]) -> usize {
    let mut regions: usize = 0;
    let mut already_counted: HashSet<(usize, usize)> = HashSet::new();

    for (row_index, row) in squares.iter().enumerate() {
        for (column_index, bit) in row.iter().enumerate() {
            if *bit == Bit::Zero || already_counted.contains(&(row_index, column_index)) {
                continue;
            }

            regions += 1;
            mark_neighbours_counted(row_index, column_index, squares, &mut already_counted);
        }
    }

    regions
}

fn mark_neighbours_counted(
    row: usize,
    col: usize,
    squares: &[Vec<Bit>],
    already_counted: &mut HashSet<(usize, usize)>,
) {
    if squares[row][col] == Bit::Zero || already_counted.contains(&(row, col)) {
        return;
    }

    already_counted.insert((row, col));
    if row > 0 {
        mark_neighbours_counted(row - 1, col, squares, already_counted);
    }

    if row < squares.len() - 1 {
        mark_neighbours_counted(row + 1, col, squares, already_counted);
    }

    if col > 0 {
        mark_neighbours_counted(row, col - 1, squares, already_counted);
    }

    if col < squares[row].len() - 1 {
        mark_neighbours_counted(row, col + 1, squares, already_counted);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref SQUARES: Vec<Vec<Bit>> = {
            construct_hashes("flqrgnkx")
        };
    }

    #[test]
    fn example_correct_bit_sequence() {
        assert_eq!(
            SQUARES[0][0..8],
            [
                Bit::One,
                Bit::One,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::Zero
            ]
        );

        assert_eq!(
            SQUARES[1][0..8],
            [
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::One
            ]
        );

        assert_eq!(
            SQUARES[2][0..8],
            [
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::One,
                Bit::Zero
            ]
        );
    }

    #[test]
    fn part_one_example() {
        assert_eq!(count_ones(&SQUARES), 8108);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(count_regions(&SQUARES), 1242);
    }

    #[test]
    fn count_two_regions() {
        let squares = vec![
            vec![Bit::One, Bit::Zero, Bit::Zero],
            vec![Bit::Zero, Bit::Zero, Bit::Zero],
            vec![Bit::Zero, Bit::One, Bit::One],
            vec![Bit::Zero, Bit::One, Bit::One],
        ];

        assert_eq!(count_regions(&squares), 2);
    }
}
