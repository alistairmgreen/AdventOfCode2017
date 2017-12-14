extern crate knot;
use knot::KnotHash;

fn main() {
    let hash = KnotHash::new("147,37,249,1,31,2,226,0,161,71,254,243,183,255,30,70");
    println!("The hash of the puzzle input is {}.", hash);
}
