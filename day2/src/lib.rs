use std::num;

pub fn read_spreadsheet(input: &str) -> Result<Vec<Vec<usize>>, num::ParseIntError> {
    input.lines().map(|line| read_numbers(&line)).collect()
}

pub fn checksum1(spreadsheet: &Vec<Vec<usize>>) -> usize {
    spreadsheet.iter().map(|row| row_checksum1(&row)).sum()
}

pub fn checksum2(spreadsheet: &Vec<Vec<usize>>) -> usize {
    spreadsheet.iter().map(|row| row_checksum2(&row)).sum()
}

fn read_numbers(input: &str) -> Result<Vec<usize>, num::ParseIntError> {
    input
        .split_whitespace()
        .map(|number| number.parse())
        .collect()
}

fn row_checksum1(numbers: &[usize]) -> usize {
    let max = numbers.iter().max().unwrap_or(&0);
    let min = numbers.iter().min().unwrap_or(&0);
    max - min
}

fn row_checksum2(numbers: &[usize]) -> usize {
    if let Some((x, y)) = find_divisible_values(numbers) {
        x / y
    }
    else {
        0
    }
}

fn find_divisible_values(numbers: &[usize]) -> Option<(usize, usize)> {
    for x in numbers.iter() {
        if let Some(y) = numbers.iter().filter(|n| **n != *x).find(|n| *x % **n == 0) {
            return Some((*x, *y));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_numbers_returns_numbers() {
        let numbers = read_numbers("1 2\t3").expect("Failed to parse the numbers");
        assert_eq!(numbers[0], 1);
        assert_eq!(numbers[1], 2);
        assert_eq!(numbers[2], 3);
    }

    #[test]
    fn read_spreadsheet_returns_matrix_of_numbers() {
        let grid = read_spreadsheet("1 2 3\n4 5 6\n7 8 9").expect("Failed to parse the numbers");
        assert_eq!(grid[0][0], 1);
        assert_eq!(grid[0][1], 2);
        assert_eq!(grid[2][2], 9);
    }

    #[test]
    fn row_checksum1_5195_is_8() {
        let checksum1 = row_checksum1(&[5, 1, 9, 5]);
        assert_eq!(checksum1, 8);
    }

    #[test]
    fn row_checksum1_753_is_4() {
        let checksum1 = row_checksum1(&[7, 5, 3]);
        assert_eq!(checksum1, 4);
    }

    #[test]
    fn row_checksum1_2468_is_6() {
        let checksum1 = row_checksum1(&[2, 4, 6, 8]);
        assert_eq!(checksum1, 6);
    }

    #[test]
    fn checksum1_correct_for_example_spreadsheet() {
        let spreadsheet = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];

        assert_eq!(checksum1(&spreadsheet), 18);
    }

    #[test]
    fn divisible_values_in_5928_are_8_and_2() {
        assert_eq!(find_divisible_values(&[5, 9, 2, 8]), Some((8, 2)));
    }

    #[test]
    fn divisible_values_in_9473_are_9_and_3() {
        assert_eq!(find_divisible_values(&[9, 4, 7, 3]), Some((9, 3)));
    }

    #[test]
    fn row_checksum2_for_5928_is_4() {
        assert_eq!(row_checksum2(&[5, 9, 2, 8]), 4);
    }

     #[test]
    fn checksum2_correct_for_example_spreadsheet() {
        let spreadsheet = vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];

        assert_eq!(checksum1(&spreadsheet), 18);
        assert_eq!(checksum2(&spreadsheet), 9);        
    }
}
