fn main() {
    let puzzle_input_lengths: Vec<u8> = vec![
        147,
        37,
        249,
        1,
        31,
        2,
        226,
        0,
        161,
        71,
        254,
        243,
        183,
        255,
        30,
        70,
    ];
    let mut numbers: Vec<u8> = (0..256).map(|n| n as u8).collect();

    knot(&mut numbers, &puzzle_input_lengths);

    let product = numbers[0] as usize * numbers[1] as usize;
    println!("The product of the first two numbers is {}", product);
}

fn get_lengths(input: &str) -> Vec<u8> {
    let mut lengths: Vec<u8> = input.to_string().into_bytes();
    let mut suffix: Vec<u8> = vec![17, 31, 73, 47, 23];
    lengths.append(&mut suffix);

    lengths
}

fn reverse_segment(numbers: &mut [u8], position: usize, segment_length: u8) {
    let numbers_length = numbers.len();
    let mut reversed: Vec<u8> = numbers
        .iter()
        .cycle()
        .skip(position)
        .take(segment_length as usize)
        .map(|&n| n)
        .collect();

    reversed.reverse();

    for (index, &n) in reversed.iter().enumerate() {
        numbers[(position + index) % numbers_length] = n;
    }
}

fn knot(numbers: &mut [u8], lengths: &[u8]) {
    let mut position = 0usize;
    let mut skip = 0usize;
    let numbers_length = numbers.len();

    for &length in lengths {
        reverse_segment(numbers, position, length);
        position = (position + length as usize + skip) % numbers_length;
        skip += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_segment_examples() {
        let mut numbers: Vec<u8> = (0..5).collect();
        reverse_segment(&mut numbers, 0, 3);
        assert_eq!(numbers, vec![2, 1, 0, 3, 4]);

        reverse_segment(&mut numbers, 3, 4);
        assert_eq!(numbers, vec![4, 3, 0, 1, 2]);
    }

    #[test]
    fn knot_example() {
        let mut numbers: Vec<u8> = (0..5).collect();
        let lengths: Vec<u8> = vec![3, 4, 1, 5];
        knot(&mut numbers, &lengths);
        assert_eq!(numbers, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn get_lengths_converts_string_correctly() {
        let lengths = get_lengths("1,2,3");
        assert_eq!(lengths, vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23]);
    }
}
