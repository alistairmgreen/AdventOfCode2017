use super::Register;
use super::errors::Error;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Instruction {
    Set(Register, Value),
    Subtract(Register, Value),
    Multiply(Register, Value),
    JumpIfNotZero(Value, Value)
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
            "set" => match arg1 {
                Value::FromRegister(r) => match arg2 {
                    Some(a) => Ok(Instruction::Set(r, a)),
                    None => Err(Error::missing_argument()),
                },
                Value::Literal(_) => Err(Error::wrong_type()),
            },
            "sub" => match arg1 {
                Value::FromRegister(r) => match arg2 {
                    Some(a) => Ok(Instruction::Subtract(r, a)),
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
            "jnz" => match arg2 {
                Some(v) => Ok(Instruction::JumpIfNotZero(arg1, v)),
                None => Err(Error::missing_argument()),
            },
            _ => Err(Error::unrecognized_instruction()),
        }
    }
}