pub mod errors;
use errors::ParseDanceMoveError;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
pub enum DanceMove {
    Spin(usize),
    Exchange((usize, usize)),
    Partner((char, char)),
}

impl FromStr for DanceMove {
    type Err = ParseDanceMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(ParseDanceMoveError::new(&format!(
                "{} is not a valid move.",
                s
            )));
        }

        let (move_type, args) = s.split_at(1);
        match move_type {
            "s" => {
                let x: usize = args.trim().parse()?;
                Ok(DanceMove::Spin(x))
            }
            "x" => {
                let arg_values = args.split('/')
                    .map(|n| n.trim().parse::<usize>())
                    .collect::<Result<Vec<usize>, ParseIntError>>()?;

                if arg_values.len() == 2 {
                    Ok(DanceMove::Exchange((arg_values[0], arg_values[1])))
                } else {
                    Err(ParseDanceMoveError::new("Missing exchange values"))
                }
            }
            "p" => {
                let arg_values: Vec<&str> = args.split('/').collect();
                if arg_values.len() == 2 {
                    if let Some(a) = arg_values[0].chars().nth(0) {
                        if let Some(b) = arg_values[1].chars().nth(0) {
                            return Ok(DanceMove::Partner((a, b)));
                        }
                    }
                }

                Err(ParseDanceMoveError::new("Missing partners"))
            }
            _ => Err(ParseDanceMoveError::new(&format!(
                "{} is not a valid move type.",
                move_type
            ))),
        }
    }
}

pub trait Dance {
    fn perform(&mut self, dance_move: &DanceMove);
}

impl Dance for Vec<char> {
    fn perform(&mut self, dance_move: &DanceMove) {
        match *dance_move {
            DanceMove::Spin(x) => spin(self, x),
            DanceMove::Exchange((a, b)) => exchange(self, a, b),
            DanceMove::Partner((a, b)) => partner(self, a, b),
        }
    }
}

pub fn letters(from: char, to: char) -> Vec<char> {
    let first: u8 = from.to_string().as_bytes()[0];
    let last: u8 = to.to_string().as_bytes()[0];
    let mut letters: Vec<char> = Vec::with_capacity((last - first) as usize);

    for letter in first..last + 1 {
        letters.push(letter.into());
    }

    letters
}

fn spin(items: &mut Vec<char>, x: usize) {
    for _ in 0..x {
        if let Some(c) = items.pop() {
            items.insert(0, c);
        }
    }
}

fn exchange(items: &mut Vec<char>, a: usize, b: usize) {
    let item_a = items[a];
    let item_b = items[b];

    items[a] = item_b;
    items[b] = item_a;
}

fn partner(items: &mut Vec<char>, a: char, b: char) {
    if let Some(index_a) = index_of(items, &a) {
        if let Some(index_b) = index_of(items, &b) {
            exchange(items, index_a, index_b);
        }
    }
}

fn index_of<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    let found = items.iter().enumerate().find(|&(_,c)| *c == *target);

    match found {
        Some((index, _)) => Some(index),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vector_of_letters() {
        assert_eq!(letters('a', 'e'), vec!['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn spin_moves_items_to_beginning() {
        let mut items = letters('a', 'e');
        spin(&mut items, 3);

        assert_eq!(items, vec!['c', 'd', 'e', 'a', 'b']);
    }

    #[test]
    fn exchange_swaps_items_by_index() {
        let mut items = letters('a', 'e');
        exchange(&mut items, 3, 4);
        assert_eq!(items, vec!['a', 'b', 'c', 'e', 'd']);
    }

    #[test]
    fn index_of_returns_character_index() {
        let items = vec!['a', 'b', 'c'];
        let index = index_of(&items, &'b').unwrap();
        assert_eq!(index, 1);
    }

    #[test]
    fn partner_swaps_items_by_value() {
        let mut items = letters('a', 'e');
        partner(&mut items, 'b', 'e');
        assert_eq!(items, vec!['a', 'e', 'c', 'd', 'b']);
    }

    #[test]
    fn perform_moves_on_vec() {
        let mut items = letters('a', 'e');
        items.perform(&DanceMove::Spin(1));
        assert_eq!(items, vec!['e', 'a', 'b', 'c', 'd']);

        items.perform(&DanceMove::Exchange((3, 4)));
        assert_eq!(items, vec!['e', 'a', 'b', 'd', 'c']);

        items.perform(&DanceMove::Partner(('e', 'b')));
        assert_eq!(items, vec!['b', 'a', 'e', 'd', 'c']);
    }

    #[test]
    fn parse_spin() {
        let spin = DanceMove::from_str("s2").unwrap();
        assert_eq!(spin, DanceMove::Spin(2));
    }

    #[test]
    fn parse_exchange() {
        let x = DanceMove::from_str("x1/2").unwrap();
        assert_eq!(x, DanceMove::Exchange((1, 2)));
    }

    #[test]
    fn parse_partner() {
        let p = DanceMove::from_str("pa/b").unwrap();
        assert_eq!(p, DanceMove::Partner(('a', 'b')));
    }
}
