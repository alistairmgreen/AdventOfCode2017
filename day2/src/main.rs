extern crate corruption_checksum;
use corruption_checksum::*;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
    }
}

fn run() -> Result<(), std::num::ParseIntError> {
    let spreadsheet = read_spreadsheet(include_str!("puzzle_input.txt"))?;
    println!("Checksum is {}", checksum(spreadsheet));

    Ok(())
}
