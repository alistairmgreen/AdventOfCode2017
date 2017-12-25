extern crate halting_problem;
use halting_problem::TuringMachine;

fn main() {
    let mut machine = TuringMachine::new();
    machine.run(12_399_302);
    let checksum = machine.checksum();
    println!("The checksum is {}.", checksum);
}
