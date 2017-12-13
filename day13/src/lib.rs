#[macro_use]
extern crate failure;

use std::collections::HashMap;
use failure::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use std::iter::FromIterator;

#[derive(Debug, Hash, Eq, PartialEq, Default)]
pub struct Scanner {
    depth: u32,
    range: u32,
}

impl Scanner {
    pub fn new(depth: u32, range: u32) -> Scanner {
        Scanner {
            depth: depth,
            range: range,
        }
    }

    pub fn severity(&self) -> u32 {
        self.depth * self.range
    }

    pub fn position(&self, time: u32) -> u32 {
        (0..self.range)
            .chain((1..self.range - 1).rev())
            .cycle()
            .nth(time as usize)
            .unwrap()
    }
}

impl FromStr for Scanner {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u32> = s.split(':')
            .map(|n| n.trim().parse())
            .collect::<Result<Vec<u32>, ParseIntError>>()?;

        if parts.len() != 2 {
            bail!("Cannot parse {} as a scanner.", s);
        }

        Ok(Scanner::new(parts[0], parts[1]))
    }
}

#[derive(Debug, Default)]
pub struct Firewall {
    scanners: HashMap<u32, Scanner>,
}

impl Firewall {
    pub fn new() -> Firewall {
        Firewall {
            scanners: HashMap::new(),
        }
    }

    pub fn add_scanner(&mut self, scanner: Scanner) {
        self.scanners.insert(scanner.depth, scanner);
    }

    pub fn caught_at_layer(&self, layer: u32) -> bool {
        if let Some(scanner) = self.scanners.get(&layer) {
            scanner.position(layer) == 0
        } else {
            false
        }
    }

    pub fn severity(&self) -> u32 {
        self.scanners.iter()
            .filter(|&(&layer, scanner)| scanner.position(layer) == 0)
            .map(|(_, scanner)| scanner.severity())
            .sum()
    }
}

impl FromIterator<Scanner> for Firewall {
    fn from_iter<I: IntoIterator<Item = Scanner>>(iter: I) -> Self {
        let mut firewall = Firewall::new();

        for i in iter {
            firewall.add_scanner(i);
        }

        firewall
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_from_string() {
        let scanner = Scanner::from_str("10: 6").unwrap();
        assert_eq!(
            scanner,
            Scanner {
                depth: 10,
                range: 6,
            }
        );
    }

    #[test]
    fn scanner_oscillates() {
        fn test_scanner(time: u32, expected: u32) {
            let position = Scanner::new(1, 4).position(time);
            assert_eq!(
                position,
                expected,
                "At time {}, expected position {} but found {}.",
                time,
                expected,
                position
            );
        }

        test_scanner(0, 0);
        test_scanner(1, 1);
        test_scanner(2, 2);
        test_scanner(3, 3);
        test_scanner(4, 2);
        test_scanner(5, 1);
        test_scanner(6, 0);
        test_scanner(7, 1);
        test_scanner(8, 2);
    }

    #[test]
    fn firewall_from_iterator() {
        let scanners = vec![Scanner::new(1, 2), Scanner::new(3, 4), Scanner::new(5, 6)];

        let firewall = scanners.into_iter().collect::<Firewall>();

        assert_eq!(firewall.scanners.len(), 3);
    }

    fn example_firewall() -> Firewall {
        let mut firewall = Firewall::new();
        firewall.add_scanner(Scanner::new(0, 3));
        firewall.add_scanner(Scanner::new(1, 2));
        firewall.add_scanner(Scanner::new(4, 4));
        firewall.add_scanner(Scanner::new(6, 4));

        firewall
    }

    #[test]
    fn caught_at_layer_0() {
        let firewall = example_firewall();
        assert!(firewall.caught_at_layer(0));
    }

    #[test]
    fn caught_at_layer_6() {
        let firewall = example_firewall();
        assert!(firewall.caught_at_layer(6));
    }

    #[test]
    fn not_caught_at_layers_1_to_5() {
        let firewall = example_firewall();

        for layer in 1..6 {
            assert!(
                !firewall.caught_at_layer(layer),
                "Should not have been caught at layer {}, but was caught.",
                layer
            );
        }
    }

    #[test]
    fn example_firewall_severity_is_24() {
        let firewall = example_firewall();
        assert_eq!(firewall.severity(), 24);
    }
}
