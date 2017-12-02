use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;

fn main() {
    if let Err(error) = run() {
        eprintln!("ERROR: {}", error);
    }
}

fn run() -> Result<(), Box<Error>> {
    let input = File::open("puzzle-input.txt")?;
    let mut reader = BufReader::new(input);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let digits: Vec<usize> = contents
        .chars()
        .filter_map(|digit| match digit {
            '0' => Some(0),
            '1' => Some(1),
            '2' => Some(2),
            '3' => Some(3),
            '4' => Some(4),
            '5' => Some(5),
            '6' => Some(6),
            '7' => Some(7),
            '8' => Some(8),
            '9' => Some(9),
            _ => None,
        })
        .collect();

    println!("CAPTCHA 1 value is {}", captcha1(&digits));
    println!("CAPTCHA 2 value is {}", captcha2(&digits));

    Ok(())
}

fn captcha1(digits: &[usize]) -> usize {
    let next_digit = digits.iter().cycle().skip(1);

    digits
        .iter()
        .zip(next_digit)
        .filter_map(|(digit, next)| if digit == next { Some(digit) } else { None })
        .sum()
}

fn captcha2(digits: &[usize]) -> usize {
    let next_digit = digits.iter().cycle().skip(digits.len() / 2);

    digits
        .iter()
        .zip(next_digit)
        .filter_map(|(digit, next)| if digit == next { Some(digit) } else { None })
        .sum()
}

#[cfg(test)]
mod tests {
    mod part1 {
        use captcha1;

        #[test]
        fn one_one_two_two_gives_three() {
            assert_eq!(captcha1(&[1, 1, 2, 2]), 3);
        }

        #[test]
        fn four_ones_gives_four() {
            assert_eq!(captcha1(&[1, 1, 1, 1]), 4);
        }

        #[test]
        fn one_two_three_four_gives_zero() {
            assert_eq!(captcha1(&[1, 2, 3, 4]), 0);
        }

        #[test]
        fn sequence_starting_and_ending_with_nine() {
            assert_eq!(captcha1(&[9, 1, 2, 1, 2, 1, 2, 9]), 9);
        }
    }

    mod part2 {
        use captcha2;

        #[test]
        fn input_1212_gives_6() {
            assert_eq!(captcha2(&[1, 2, 1, 2]), 6);
        }

        #[test]
        fn input_1221_gives_0() {
            assert_eq!(captcha2(&[1, 2, 2, 1]), 0);
        }

        #[test]
        fn input_123425_gives_4() {
            assert_eq!(captcha2(&[1, 2, 3, 4, 2, 5]), 4);
        }

        #[test]
        fn input_123123_gives_12() {
            assert_eq!(captcha2(&[1, 2, 3, 1, 2, 3]), 12);
        }

        #[test]
        fn input_12131415_gives_4() {
            assert_eq!(captcha2(&[1, 2, 1, 3, 1, 4, 1, 5]), 4);
        }
    }
}
