#[derive(Debug)]
pub struct Generator {
    value: usize,
    factor: usize,
}

impl Generator {
    pub fn type_a(seed: usize) -> Generator {
        Generator {
            value: seed,
            factor: 16_807,
        }
    }

    pub fn type_b(seed: usize) -> Generator {
        Generator {
            value: seed,
            factor: 48_271,
        }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.factor) % 2_147_483_647;
        Some(self.value)
    }
}

pub fn lower_16_bits(number: usize) -> u16 {
    (number & 0xFFFF) as u16
}

pub fn matches_part_two(seed_a: usize, seed_b: usize) -> usize {
    let generator_a = Generator::type_a(seed_a).filter(|&n| n % 4 == 0);

    let generator_b = Generator::type_b(seed_b).filter(|&n| n % 8 == 0);

    generator_a
        .zip(generator_b)
        .take(5_000_000)
        .filter(|&(a, b)| lower_16_bits(a) == lower_16_bits(b))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_a_example() {
        let values: Vec<usize> = Generator::type_a(65).take(5).collect();

        assert_eq!(
            values,
            vec![1092455, 1181022009, 245556042, 1744312007, 1352636452]
        );
    }

    #[test]
    fn generator_b_example() {
        let values: Vec<usize> = Generator::type_b(8921).take(5).collect();

        assert_eq!(
            values,
            vec![430625591, 1233683848, 1431495498, 137874439, 285222916]
        );
    }

    #[test]
    fn lower_16_bits_equal() {
        assert_eq!(lower_16_bits(245556042), lower_16_bits(1431495498));
    }

    #[test]
    fn example_pairs() {
        let matches = Generator::type_a(65)
            .zip(Generator::type_b(8921))
            .take(40_000_000)
            .filter(|&(a, b)| lower_16_bits(a) == lower_16_bits(b))
            .count();

        assert_eq!(matches, 588);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(matches_part_two(65, 8921), 309);
    }
}
