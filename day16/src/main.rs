extern crate permutation_promenade;
use permutation_promenade::*;
use permutation_promenade::errors::*;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run() -> Result<(), ParseDanceMoveError> {
    let input = include_str!("puzzle_input.txt");
    let original_arrangement = letters('a', 'p');
    let mut arrangement = original_arrangement.clone();

    let dance_moves = input
        .split(',')
        .map(|item| item.parse::<DanceMove>())
        .collect::<Result<Vec<DanceMove>, ParseDanceMoveError>>()?;
    
    let mut dance_count: usize = 1;
    do_dance(&mut arrangement, &dance_moves);

    println!("After one dance:");
    print_arrangement(&arrangement);

    while arrangement != original_arrangement {
        do_dance(&mut arrangement, &dance_moves);
        dance_count += 1;    
    }

    println!("The original arrangement is restored after {} dances.", dance_count);

    let equivalent_of_a_billion = 1_000_000_000 % dance_count;
    println!("A billion dances are therefore equivalent to {} dances.", equivalent_of_a_billion);
    
    for _ in 0..equivalent_of_a_billion {
        do_dance(&mut arrangement, &dance_moves);
    }

    println!("Final arrangement:");
    print_arrangement(&arrangement);
    
    Ok(())
}

fn do_dance(arrangement: &mut Vec<char>, dance_moves: &[DanceMove]) {
    for step in dance_moves {
        arrangement.perform(step);
    }
}

fn print_arrangement(programs: &[char]) {
    for letter in programs {
        print!("{}", letter);
    }
    println!();
}
