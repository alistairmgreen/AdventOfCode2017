use std::num;

pub fn read_spreadsheet(input: &str) -> Result<Vec<Vec<usize>>, num::ParseIntError> {
    input.lines().map(|line| read_numbers(&line)).collect()
}

pub fn checksum(spreadsheet: Vec<Vec<usize>>) -> usize {
    spreadsheet.iter().map(|row| row_checksum(&row)).sum()
}

fn read_numbers(input: &str) -> Result<Vec<usize>, num::ParseIntError> {
    input
        .split_whitespace()
        .map(|number| number.parse())
        .collect()
}

fn row_checksum(numbers: &[usize]) -> usize {
    let max = numbers.iter().max().unwrap_or(&0);
    let min = numbers.iter().min().unwrap_or(&0);
    max - min
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
    fn row_checksum_5195_is_8() {
        let checksum = row_checksum(&[5, 1, 9, 5]);
        assert_eq!(checksum, 8);
    }

    #[test]
    fn row_checksum_753_is_4() {
        let checksum = row_checksum(&[7, 5, 3]);
        assert_eq!(checksum, 4);
    }

    #[test]
    fn row_checksum_2468_is_6() {
        let checksum = row_checksum(&[2, 4, 6, 8]);
        assert_eq!(checksum, 6);
    }

    #[test]
    fn checksum_correct_for_example_spreadsheet() {
        let spreadsheet = vec!(
            vec!(5, 1, 9, 5),
            vec!(7, 5, 3),
            vec!(2, 4, 6, 8)
        );

        assert_eq!(checksum(spreadsheet), 18);
    }
}
