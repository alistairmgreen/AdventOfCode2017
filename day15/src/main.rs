extern crate generators;
use generators::*;

fn main() {
    let seed_a = 634;
    let seed_b = 301;

    let matches = Generator::type_a(seed_a)
        .zip(Generator::type_b(seed_b))
        .take(40_000_000)
        .filter(|&(a, b)| lower_16_bits(a) == lower_16_bits(b))
        .count();
    
    println!("Part 1: After 40 million iterations, {} pairs were found with matching lower 16 bits.", matches);

    println!("Part 2: After 5 million iterations, {} pairs were found with matching lower 16 bits.", matches_part_two(seed_a, seed_b));
}
