extern crate generators;
use generators::*;

fn main() {
    let matches = Generator::type_a(634)
        .zip(Generator::type_b(301))
        .take(40_000_000)
        .filter(|&(a, b)| lower_16_bits(a) == lower_16_bits(b))
        .count();
    
    println!("After 40 million iterations, {} pairs were found with matching lower 16 bits.", matches);
}
