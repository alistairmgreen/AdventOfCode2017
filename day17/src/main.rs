fn main() {
    let step_size = 367; // The puzzle input
    let (index, buffer) = spinlock(step_size, 2017);
    let next_number = buffer[(index + 1) % buffer.len()];
    println!("The number after 2017 is {}.", next_number);
}

fn spinlock(step: usize, iterations: usize) -> (usize, Vec<usize>) {
    let mut buffer: Vec<usize> = Vec::with_capacity(iterations + 1);
    buffer.push(0);
    let mut index = 0;

    for value in 1..iterations + 1 {
        index = ((index + step) % buffer.len()) + 1;
        buffer.insert(index, value);
    }

    (index, buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_step_1() {
        let (index, buffer) = spinlock(3, 1);
        assert_eq!(index, 1);
        assert_eq!(buffer, vec![0, 1]);
    }

    #[test]
    fn example_step_2() {
        let (index, buffer) = spinlock(3, 2);
        assert_eq!(index, 1);
        assert_eq!(buffer, vec![0, 2, 1]);
    }

    #[test]
    fn example_step_3() {
        let (index, buffer) = spinlock(3, 3);
        assert_eq!(index, 2);
        assert_eq!(buffer, vec![0, 2, 3, 1]);
    }

    #[test]
    fn number_after_2017_is_638() {
        let (index, buffer) = spinlock(3, 2017);
        let next_number = buffer[(index + 1) % buffer.len()];
        assert_eq!(next_number, 638);
    }
}
