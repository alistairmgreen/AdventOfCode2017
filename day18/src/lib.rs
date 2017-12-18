use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

pub type Register = char;

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    UnrecognizedInstruction,
    MissingArgument,
    WrongArgumentType,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    fn missing_argument() -> Error {
        Error {
            kind: ErrorKind::MissingArgument,
        }
    }

    fn unrecognized_instruction() -> Error {
        Error {
            kind: ErrorKind::UnrecognizedInstruction,
        }
    }

    fn wrong_type() -> Error {
        Error {
            kind: ErrorKind::WrongArgumentType,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::UnrecognizedInstruction => write!(f, "Unrecognized instruction"),
            ErrorKind::MissingArgument => write!(f, "Missing argument"),
            ErrorKind::WrongArgumentType => {
                write!(f, "Literal value supplied where register required")
            }
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "Invalid instruction"
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    FromRegister(Register),
    Literal(i64),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse() {
            Ok(Value::Literal(n))
        } else if let Some(c) = s.chars().nth(0) {
            Ok(Value::FromRegister(c))
        } else {
            Err(Error::missing_argument())
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Add(Register, Value),
    Modulus(Register, Value),
    Multiply(Register, Value),
    Recover(Value),
    Set(Register, Value),
    Sound(Value),
    JumpIfGreaterThanZero(Value, Value),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let length = parts.len();
        if length < 2 {
            return Err(Error::missing_argument());
        }

        let instruction = parts[0];
        let arg1: Value = parts[1].parse()?;

        let arg2: Option<Value> = if length > 2 {
            Some(parts[2].parse()?)
        } else {
            None
        };

        match instruction {
            "add" => match arg1 {
                Value::FromRegister(r) => match arg2 {
                    Some(a) => Ok(Instruction::Add(r, a)),
                    None => Err(Error::missing_argument()),
                },
                Value::Literal(_) => Err(Error::wrong_type()),
            },
            "mod" => match arg1 {
                Value::FromRegister(r) => match arg2 {
                    Some(a) => Ok(Instruction::Modulus(r, a)),
                    None => Err(Error::missing_argument()),
                },
                Value::Literal(_) => Err(Error::wrong_type()),
            },
            "mul" => match arg1 {
                Value::FromRegister(r) => match arg2 {
                    Some(a) => Ok(Instruction::Multiply(r, a)),
                    None => Err(Error::missing_argument()),
                },
                Value::Literal(_) => Err(Error::wrong_type()),
            },
            "rcv" => Ok(Instruction::Recover(arg1)),
            "set" => match arg1 {
                Value::FromRegister(r) => match arg2 {
                    Some(a) => Ok(Instruction::Set(r, a)),
                    None => Err(Error::missing_argument()),
                },
                Value::Literal(_) => Err(Error::wrong_type()),
            },
            "snd" => Ok(Instruction::Sound(arg1)),
            "jgz" => match arg2 {
                Some(v) => Ok(Instruction::JumpIfGreaterThanZero(arg1, v)),
                None => Err(Error::missing_argument()),
            },
            _ => Err(Error::unrecognized_instruction()),
        }
    }
}


struct Registers {
    registers: HashMap<Register, i64>,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            registers: HashMap::new(),
        }
    }

    fn get_value(&self, value: &Value) -> i64 {
        match *value {
            Value::FromRegister(r) => *self.registers.get(&r).unwrap_or(&0),
            Value::Literal(v) => v,
        }
    }

    fn get_mut(&mut self, register: &Register) -> &mut i64 {
        self.registers.entry(register.clone()).or_insert(0)
    }
}

pub fn play(instructions: &[Instruction]) -> Option<i64> {
    let mut registers = Registers::new();
    let mut last_sound: Option<i64> = None;
    let mut index = 0;

    while index < instructions.len() {
        match instructions[index] {
            Instruction::Add(ref register, ref value) => {
                *registers.get_mut(register) += registers.get_value(value);
            }
            Instruction::Modulus(ref register, ref value) => {
                let modulo = registers.get_value(value);
                *registers.get_mut(register) %= modulo;
            }
            Instruction::Multiply(ref register, ref value) => {
                let factor = registers.get_value(value);
                *registers.get_mut(register) *= factor;
            }
            Instruction::Recover(ref value) => {
                if registers.get_value(value) > 0 {
                    return last_sound;
                }
            }
            Instruction::Set(ref register, ref value) => {
                *registers.get_mut(register) = registers.get_value(value);
            }
            Instruction::Sound(ref value) => {
                last_sound = Some(registers.get_value(value));
            }
            Instruction::JumpIfGreaterThanZero(_, _) => {}
        }

        let increment = match instructions[index] {
            Instruction::JumpIfGreaterThanZero(ref condition, ref jump) => {
                if registers.get_value(condition) > 0 {
                    registers.get_value(jump)
                } else {
                    1
                }
            }
            _ => 1,
        };

        index = (index as i64 + increment) as usize;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_example() {
        let instructions = vec![
            Instruction::Set('a', Value::Literal(1)),
            Instruction::Add('a', Value::Literal(2)),
            Instruction::Multiply('a', Value::FromRegister('a')),
            Instruction::Modulus('a', Value::Literal(5)),
            Instruction::Sound(Value::FromRegister('a')),
            Instruction::Set('a', Value::Literal(0)),
            Instruction::Recover(Value::FromRegister('a')),
            Instruction::JumpIfGreaterThanZero(Value::FromRegister('a'), Value::Literal(-1)),
            Instruction::Set('a', Value::Literal(1)),
            Instruction::JumpIfGreaterThanZero(Value::FromRegister('a'), Value::Literal(-2)),
        ];

        assert_eq!(play(&instructions).unwrap(), 4);
    }

    #[test]
    fn parse_add_valid() {
        let add: Instruction = "add a 2".parse().expect("'add a 2' is a valid instruction");
        assert_eq!(add, Instruction::Add('a', Value::Literal(2)));
    }
}
