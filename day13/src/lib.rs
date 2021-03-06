#[macro_use]
extern crate failure;

use std::collections::HashMap;
use failure::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use std::iter::FromIterator;

#[derive(Debug, Hash, Eq, PartialEq, Default)]
pub struct Scanner {
    depth: usize,
    range: usize,
}

impl Scanner {
    pub fn new(depth: usize, range: usize) -> Scanner {
        Scanner {
            depth: depth,
            range: range,
        }
    }

    pub fn severity(&self) -> usize {
        self.depth * self.range
    }

    pub fn position(&self, time: usize) -> usize {
        let max_index = self.range - 1;
        let t = time % (2 * max_index);

        if t > max_index {
            max_index - (t - max_index)
        } else {
            t
        }
    }
}

impl FromStr for Scanner {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = s.split(':')
            .map(|n| n.trim().parse())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;

        if parts.len() != 2 {
            bail!("Cannot parse {} as a scanner.", s);
        }

        Ok(Scanner::new(parts[0], parts[1]))
    }
}

#[derive(Debug, Default)]
pub struct Firewall {
    scanners: HashMap<usize, Scanner>,
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

    pub fn caught_at_layer(&self, layer: usize) -> bool {
        if let Some(scanner) = self.scanners.get(&layer) {
            scanner.position(layer) == 0
        } else {
            false
        }
    }

    pub fn severity(&self) -> usize {
        self.scanners
            .iter()
            .filter(|&(&layer, scanner)| scanner.position(layer) == 0)
            .map(|(_, scanner)| scanner.severity())
            .sum()
    }

    pub fn caught_at_time_delay(&self, delay: usize) -> bool {
        self.scanners
            .iter()
            .any(|(&layer, scanner)| scanner.position(layer + delay) == 0)
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
        fn test_scanner(time: usize, expected: usize) {
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
    fn caught_at_time_delay_0_to_9() {
        let firewall = example_firewall();

        for t in 0..10 {
            assert!(
                firewall.caught_at_time_delay(t),
                "Should be caught with time delay = {}, but was not caught.",
                t
            );
        }
    }

    #[test]
    fn not_caught_at_time_delay_10() {
        let firewall = example_firewall();
        assert!(!firewall.caught_at_time_delay(10));
    }

    #[test]
    fn example_firewall_severity_is_24() {
        let firewall = example_firewall();
        assert_eq!(firewall.severity(), 24);
    }

    #[test]
    fn find_time_to_escape_capture() {
        let firewall = example_firewall();
        let delay = (0..100)
            .find(|&delay| !firewall.caught_at_time_delay(delay))
            .unwrap();
        assert_eq!(delay, 10);
    }
}
