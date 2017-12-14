#[derive(Debug, Eq, PartialEq)]
pub enum Bit {
    Zero,
    One,
}

pub trait AsBits {
    fn as_bits(&self) -> Vec<Bit>;
}

impl AsBits for u8 {
    fn as_bits(&self) -> Vec<Bit> {
        let mut bits: Vec<Bit> = Vec::with_capacity(8);

        for n in (0..8).rev() {
            let bit = if self & (1 << n) == 0 {
                Bit::Zero
            } else {
                Bit::One
            };

            bits.push(bit);
        }

        bits
    }
}

impl<'a> AsBits for &'a[u8] {
    fn as_bits(&self) -> Vec<Bit> {
        let mut bits: Vec<Bit> = Vec::with_capacity(self.len() * 8);

        for &byte in self.iter() {
            bits.append(&mut byte.as_bits());
        }

        bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_zero_as_bits() {
        let bits = 0u8.as_bits();
        assert_eq!(bits.len(), 8);

        for n in 0..8 {
            assert_eq!(bits[n], Bit::Zero);
        }
    }

    #[test]
    fn u8_one_as_bits() {
        let bits = 1u8.as_bits();
        assert_eq!(
            bits,
            vec![
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
            ]
        );
    }

    #[test]
    fn u8_two_as_bits() {
        let bits = 2u8.as_bits();
        assert_eq!(
            bits,
            vec![
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
            ]
        );
    }

    #[test]
    fn u8_255_as_bits() {
        let bits = 255u8.as_bits();
        assert_eq!(
            bits,
            vec![
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
            ]
        );
    }

    #[test]
    fn byte_slice_as_bits() {
        let input_bytes = vec![255u8, 2u8];
        assert_eq!(
            input_bytes.as_slice().as_bits(),
            vec![
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                // 2nd byte
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
            ]
        );
    }
}
