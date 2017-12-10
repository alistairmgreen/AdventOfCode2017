fn main() {
    let puzzle_input_lengths: Vec<usize> = vec![
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
    let mut numbers: Vec<usize> = (0..256).collect();

    knot(&mut numbers, &puzzle_input_lengths);

    let product = numbers[0] * numbers[1];
    println!("The product of the first two numbers is {}", product);
}

fn reverse_segment(numbers: &mut [usize], position: usize, segment_length: usize) {
    let numbers_length = numbers.len();
    let mut reversed: Vec<usize> = numbers
        .iter()
        .cycle()
        .skip(position)
        .take(segment_length)
        .map(|&n| n)
        .collect();

    reversed.reverse();

    for (index, &n) in reversed.iter().enumerate() {
        numbers[(position + index) % numbers_length] = n;
    }
}

fn knot(numbers: &mut [usize], lengths: &[usize]) {
    let mut position = 0usize;
    let mut skip = 0usize;
    let numbers_length = numbers.len();

    for &length in lengths {
        reverse_segment(numbers, position, length);
        position = (position + length + skip) % numbers_length;
        skip += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_segment_examples() {
        let mut numbers: Vec<usize> = (0..5).collect();
        reverse_segment(&mut numbers, 0, 3);
        assert_eq!(numbers, vec![2usize, 1usize, 0usize, 3usize, 4usize]);

        reverse_segment(&mut numbers, 3, 4);
        assert_eq!(numbers, vec![4usize, 3usize, 0usize, 1usize, 2usize]);
    }

    #[test]
    fn knot_example() {
        let mut numbers: Vec<usize> = (0..5).collect();
        let lengths: Vec<usize> = vec![3, 4, 1, 5];
        knot(&mut numbers, &lengths);
        let expected: Vec<usize> = vec![3, 4, 2, 1, 0];
        assert_eq!(numbers, expected);
    }
}
