use std::fmt;

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

fn knot(numbers: &mut [u8], lengths: &[u8], position: &mut usize, skip: &mut usize) {
    let numbers_length = numbers.len();

    for &length in lengths {
        reverse_segment(numbers, *position, length);
        *position = (*position + length as usize + *skip) % numbers_length;
        *skip += 1;
    }
}

fn sparse_hash(input_numbers: &[u8], lengths: &[u8]) -> Vec<u8> {
    let mut position: usize = 0;
    let mut skip: usize = 0;
    let mut numbers = input_numbers.to_vec();

    for _ in 0..64 {
        knot(&mut numbers, &lengths, &mut position, &mut skip);
    }

    numbers
}

fn dense_hash(input: &[u8]) -> Vec<u8> {
    input.chunks(16usize).map(|chunk| xor_all(chunk)).collect()
}

fn xor_all(chunk: &[u8]) -> u8 {
    chunk.iter().skip(1).fold(chunk[0], |acc, x| acc ^ x)
}

#[derive(Debug)]
pub struct KnotHash {
    value: Vec<u8>,
}

impl KnotHash {
    pub fn new(input: &str) -> KnotHash {
        let lengths = get_lengths(input);
        let numbers: Vec<u8> = (0..256).map(|n| n as u8).collect();
        let hash = dense_hash(&sparse_hash(&numbers, &lengths));

        KnotHash { value: hash }
    }
}

impl fmt::Display for KnotHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.value.iter() {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
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
        knot(&mut numbers, &lengths, &mut 0, &mut 0);
        assert_eq!(numbers, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn get_lengths_converts_string_correctly() {
        let lengths = get_lengths("1,2,3");
        assert_eq!(lengths, vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23]);
    }

    #[test]
    fn xor_all_works_correctly() {
        let input: Vec<u8> = vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        assert_eq!(xor_all(&input), 64u8);
    }

    #[test]
    fn knot_hash_formatting() {
        let hash = KnotHash {
            value: vec![64, 7, 255]
        };

        assert_eq!(hash.to_string(), "4007ff");
    }    

    #[test]
    fn hash_empty_string() {
        let hash = KnotHash::new("").to_string();
        assert_eq!(hash, "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn hash_aoc2017() {
        let hash = KnotHash::new("AoC 2017").to_string();
        assert_eq!(hash, "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn hash_123() {
        let hash = KnotHash::new("1,2,3").to_string();
        assert_eq!(hash, "3efbe78a8d82f29979031a4aa0b16a9d");
    }

     #[test]
    fn hash_124() {
        let hash = KnotHash::new("1,2,4").to_string();
        assert_eq!(hash, "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
