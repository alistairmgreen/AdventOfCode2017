#[macro_use]
extern crate failure;

use std::collections::HashMap;
use failure::Error;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Operation {
    Increment,
    Decrement,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Operation::Increment),
            "dec" => Ok(Operation::Decrement),
            _ => bail!("Unrecognized operator '{}'.", s),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Instruction {
    register: String,
    operation: Operation,
    argument: i32,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 3 {
            bail!("{} is not a valid statement", s);
        }

        let operation = Operation::from_str(&parts[1])?;
        let argument = i32::from_str(&parts[2])?;

        Ok(Instruction {
            register: parts[0].to_string(),
            operation: operation,
            argument: argument,
        })
    }
}

impl Instruction {
    fn increment_value(&self) -> i32 {
        match self.operation {
            Operation::Increment => self.argument,
            Operation::Decrement => -self.argument,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Comparison {
    Less,
    LessOrEqual,
    Equal,
    NotEqual,
    GreaterOrEqual,
    Greater,
}

impl FromStr for Comparison {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let comparison = match s {
            "<" => Comparison::Less,
            "<=" => Comparison::LessOrEqual,
            "==" => Comparison::Equal,
            "!=" => Comparison::NotEqual,
            ">=" => Comparison::GreaterOrEqual,
            ">" => Comparison::Greater,
            _ => bail!("Unrecognized operator '{}'.", s),
        };

        Ok(comparison)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Condition {
    register: String,
    comparison: Comparison,
    argument: i32,
}

impl Condition {
    pub fn holds_for_value(&self, register_value: i32) -> bool {
        match self.comparison {
            Comparison::Less => register_value < self.argument,
            Comparison::LessOrEqual => register_value <= self.argument,
            Comparison::Equal => register_value == self.argument,
            Comparison::NotEqual => register_value != self.argument,
            Comparison::GreaterOrEqual => register_value >= self.argument,
            Comparison::Greater => register_value > self.argument,
        }
    }
}

impl FromStr for Condition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 3 {
            bail!("{} is not a valid condition", s);
        }

        let comparison = Comparison::from_str(&parts[1])?;
        let argument = i32::from_str(&parts[2])?;

        Ok(Condition {
            register: parts[0].to_string(),
            comparison: comparison,
            argument: argument,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Statement {
    instruction: Instruction,
    condition: Condition,
}

impl FromStr for Statement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" if ").collect();
        if parts.len() < 2 {
            bail!("{} is not a valid condition", s);
        }

        let instruction = Instruction::from_str(&parts[0])?;
        let condition = Condition::from_str(&parts[1])?;

        Ok(Statement {
            instruction: instruction,
            condition: condition,
        })
    }
}

#[derive(Debug)]
pub struct Registers {
    registers: HashMap<String, i32>,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: HashMap::new(),
        }
    }

    fn get(&mut self, register: &String) -> &mut i32 {
        self.registers.entry(register.clone()).or_insert(0)
    }

    pub fn execute(&mut self, statement: &Statement) {
        let condition_value = *self.get(&statement.condition.register);
        if statement.condition.holds_for_value(condition_value) {
            let register_value = self.get(&statement.instruction.register);
            *register_value += statement.instruction.increment_value();
        }
    }

    pub fn largest_value(&self) -> i32 {
        *self.registers.values().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_statement_if_condition_met() {
        let statement = Statement {
            instruction: Instruction {
                register: "a".to_string(),
                operation: Operation::Increment,
                argument: 1,
            },
            condition: Condition {
                register: "b".to_string(),
                comparison: Comparison::Less,
                argument: 5,
            },
        };

        let mut registers = Registers::new();
        registers.execute(&statement);

        assert_eq!(registers.registers["a"], 1);
    }

    #[test]
    fn do_not_execute_statement_if_condition_not_met() {
        let statement = Statement {
            instruction: Instruction {
                register: "b".to_string(),
                operation: Operation::Increment,
                argument: 5,
            },
            condition: Condition {
                register: "a".to_string(),
                comparison: Comparison::Greater,
                argument: 1,
            },
        };

        let mut registers = Registers::new();
        registers.execute(&statement);

        assert_eq!(*registers.get(&"b".to_string()), 0)
    }

    #[test]
    fn part_1_example() {
        let statements = vec![
            Statement {
                instruction: Instruction {
                    register: "b".to_string(),
                    operation: Operation::Increment,
                    argument: 5,
                },
                condition: Condition {
                    register: "a".to_string(),
                    comparison: Comparison::Greater,
                    argument: 1,
                },
            },
            Statement {
                instruction: Instruction {
                    register: "a".to_string(),
                    operation: Operation::Increment,
                    argument: 1,
                },
                condition: Condition {
                    register: "b".to_string(),
                    comparison: Comparison::Less,
                    argument: 5,
                },
            },
            Statement {
                instruction: Instruction {
                    register: "c".to_string(),
                    operation: Operation::Decrement,
                    argument: -10,
                },
                condition: Condition {
                    register: "a".to_string(),
                    comparison: Comparison::GreaterOrEqual,
                    argument: 1,
                },
            },
            Statement {
                instruction: Instruction {
                    register: "c".to_string(),
                    operation: Operation::Increment,
                    argument: -20,
                },
                condition: Condition {
                    register: "c".to_string(),
                    comparison: Comparison::Equal,
                    argument: 10,
                },
            },
        ];

        let mut registers = Registers::new();
        for statement in statements.iter() {
            registers.execute(&statement);
        }

        assert_eq!(registers.largest_value(), 1);
    }

    #[test]
    fn parse_operation_increment() {
        assert_eq!("inc".parse::<Operation>().unwrap(), Operation::Increment);
    }

    #[test]
    fn parse_operation_decrement() {
        assert_eq!("dec".parse::<Operation>().unwrap(), Operation::Decrement);
    }

    #[test]
    fn parse_statement() {
        let statement = Statement::from_str("c dec -10 if a >= 1").unwrap();
        assert_eq!(
            statement.instruction,
            Instruction {
                register: "c".to_string(),
                operation: Operation::Decrement,
                argument: -10,
            }
        );

        assert_eq!(
            statement.condition,
            Condition {
                register: "a".to_string(),
                comparison: Comparison::GreaterOrEqual,
                argument: 1
            }
        );
    }
}
