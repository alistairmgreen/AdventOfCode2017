#[macro_use]
extern crate failure;

use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use failure::Error;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Program {
    pub name: String,
    pub weight: u32,
    pub children: Vec<String>,
}

impl Program {
    pub fn new(name: &str, weight: u32) -> Program {
        Program {
            name: name.to_string(),
            weight: weight,
            children: Vec::with_capacity(0),
        }
    }

    pub fn with_children(name: &str, weight: u32, children: &[&str]) -> Program {
        Program {
            name: name.to_string(),
            weight: weight,
            children: children.iter().map(|&s| s.to_string()).collect(),
        }
    }
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let field_count = parts.len();
        if field_count < 2 {
            bail!("Not enough fields");
        }

        let name = parts[0];
        let weight: u32 = parts[1].trim_matches(|c| c == '(' || c == ')').parse()?;

        if field_count > 3 {
            let children: Vec<&str> = parts[3..].iter().map(|s| s.trim_matches(',')).collect();
            return Ok(Program::with_children(name, weight, &children));
        }

        Ok(Program::new(name, weight))
    }
}

pub fn find_root<'a>(programs: &'a HashMap<String, Program>) -> &'a Program {
    let programs_with_children: Vec<&Program> = programs
        .values()
        .filter(|&p| !p.children.is_empty())
        .collect();

    let mut child_nodes: HashSet<&String> = HashSet::new();
    for program in &programs_with_children {
        for child in &program.children {
            child_nodes.insert(child);
        }
    }

    programs_with_children
        .iter()
        .find(|&p| !child_nodes.contains(&p.name))
        .unwrap()
}

pub fn total_weight(program_name: &str, all_programs: &HashMap<String, Program>) -> u32 {
    let program = &all_programs[program_name];

    program.weight
        + program
            .children
            .iter()
            .map(|name| total_weight(name, &all_programs))
            .sum::<u32>()
}

pub fn is_balanced(program_name: &str, all_programs: &HashMap<String, Program>) -> bool {
    let program = &all_programs[program_name];
    if program.children.is_empty() {
        return true;
    }

    let child_weights: Vec<u32> = program
        .children
        .iter()
        .map(|name| total_weight(name, &all_programs))
        .collect();

    for &weight in child_weights.iter() {
        if weight != child_weights[0] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_data() -> HashMap<String, Program> {
        vec![
            Program::new("pbga", 66),
            Program::new("xhth", 57),
            Program::new("ebii", 61),
            Program::new("havc", 66),
            Program::new("ktlj", 57),
            Program::with_children("fwft", 72, &["ktlj", "cntj", "xhth"]),
            Program::new("qoyq", 66),
            Program::with_children("padx", 45, &["pbga", "havc", "qoyq"]),
            Program::with_children("tknk", 41, &["ugml", "padx", "fwft"]),
            Program::new("jptl", 61),
            Program::with_children("ugml", 68, &["gyxo", "ebii", "jptl"]),
            Program::new("gyxo", 61),
            Program::new("cntj", 57),
        ].into_iter()
            .map(|p| (p.name.clone(), p))
            .collect()
    }

    #[test]
    fn program_from_string_without_children() {
        let program =
            Program::from_str("pbga (66)").expect("String should be parseable as a Program");

        assert_eq!(program.name, "pbga".to_string());
        assert_eq!(program.weight, 66);
        assert!(program.children.is_empty());
    }

    #[test]
    fn program_from_string_with_children() {
        let program = "fwft (72) -> ktlj, cntj, xhth"
            .parse::<Program>()
            .expect("String should be parseable as a Program");

        assert_eq!(
            program,
            Program::with_children("fwft", 72, &["ktlj", "cntj", "xhth"])
        );
    }

    #[test]
    fn find_root_correct_for_example_data() {
        let programs = example_data();
        let root = find_root(&programs);
        assert_eq!(root.name, "tknk".to_string());
    }

    #[test]
    fn total_weight_is_weight_of_self_plus_all_children() {
        let programs = example_data();
        assert_eq!(total_weight("gyxo", &programs), 61);
        assert_eq!(total_weight("ugml", &programs), 251);
    }

    #[test]
    fn program_with_no_children_is_balanced() {
        let programs = example_data();
        assert!(is_balanced("gyxo", &programs));
    }

    #[test]
    fn program_whose_children_have_equal_weights_is_balanced() {
        let programs = example_data();
        assert!(is_balanced("ugml", &programs));
    }

    #[test]
    fn program_whose_children_have_different_weights_is_imbalanced() {
        let programs = example_data();
        assert!(!is_balanced("tknk", &programs));
    }
}
