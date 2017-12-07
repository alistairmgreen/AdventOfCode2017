use std::collections::HashSet;

fn main() {
    let mut memory_bank: Vec<u8> = vec![0, 5, 10, 0, 11, 14, 13, 4, 11, 8, 8, 7, 1, 4, 12, 11];

    let mut previous_states: HashSet<Vec<u8>> = HashSet::new();
    let mut cycles: usize = 0;

    loop {
        cycles += 1;
        previous_states.insert(memory_bank.clone());
        redistribute(&mut memory_bank);
        if previous_states.contains(&memory_bank) {
            break;
        }
    }

    println!("A repetition was found after {} cycles.", cycles);

    let first_repeated_state = memory_bank.clone();

    cycles = 1;
    redistribute(&mut memory_bank);
    while memory_bank != first_repeated_state {
        redistribute(&mut memory_bank);
        cycles += 1;
    }

    println!("The same sequence appeared again after {} cycles.", cycles);
}

fn index_of_largest(numbers: &[u8]) -> usize {
    let mut largest_value: u8 = numbers[0];
    let mut largest_index: usize = 0;

    for (index, &n) in numbers.iter().enumerate() {
        if n > largest_value {
            largest_value = n;
            largest_index = index as usize;
        }
    }

    largest_index
}

fn redistribute(numbers: &mut [u8]) {
    let length = numbers.len();
    let mut index = index_of_largest(numbers);
    let mut remaining_blocks = numbers[index];
    numbers[index] = 0;
    while remaining_blocks > 0 {
        index = (index + 1) % length;
        numbers[index] += 1;
        remaining_blocks -= 1;
    }
}
