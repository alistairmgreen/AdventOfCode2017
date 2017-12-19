use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use std::thread;

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
    Add(Register, Value),
    Modulus(Register, Value),
    Multiply(Register, Value),
    Receive(Register),
    Set(Register, Value),
    Send(Value),
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
            "rcv" => match arg1 {
                Value::FromRegister(r) => Ok(Instruction::Receive(r)),
                Value::Literal(_) => Err(Error::wrong_type())
            },
            "set" => match arg1 {
                Value::FromRegister(r) => match arg2 {
                    Some(a) => Ok(Instruction::Set(r, a)),
                    None => Err(Error::missing_argument()),
                },
                Value::Literal(_) => Err(Error::wrong_type()),
            },
            "snd" => Ok(Instruction::Send(arg1)),
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

    fn with_id(id: i64) -> Registers {
        let mut map = HashMap::new();
        map.insert('p', id);

        Registers {
            registers: map
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
            Instruction::Receive(ref register) => {
                if *registers.get_mut(register) > 0 {
                    return last_sound;
                }
            }
            Instruction::Set(ref register, ref value) => {
                *registers.get_mut(register) = registers.get_value(value);
            }
            Instruction::Send(ref value) => {
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

fn count_messages(instructions: &[Instruction], id: i64, tx: Sender<i64>, rx: Receiver<i64>) {
    let mut messages_sent: usize = 0;

    let mut registers = Registers::with_id(id);
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
            Instruction::Receive(ref register) => {
               match rx.recv_timeout(Duration::from_secs(1)) {
                   Ok(value) => *registers.get_mut(register) = value,
                   Err(_) => {
                       println!("Receive timed out for program {}; terminating.", id);
                       break;
                   }
               }
            }
            Instruction::Set(ref register, ref value) => {
                *registers.get_mut(register) = registers.get_value(value);
            }
            Instruction::Send(ref value) => {
                let to_send = registers.get_value(value);
                match tx.send(to_send) {
                    Ok(_) => messages_sent += 1,
                    Err(_) => {
                        println!("Program {} cannot send; terminating.", id);
                        break; // Assume the other thread has hung up
                    }
                }
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

    println!("Program {} has sent {} messages.", id, messages_sent);
}

pub fn perform_duet(instructions: &[Instruction]) {
    let mut instructions_0: Vec<Instruction> = Vec::with_capacity(instructions.len());
    let mut instructions_1: Vec<Instruction> = Vec::with_capacity(instructions.len());
    for instruction in instructions.iter() {
        instructions_0.push(instruction.clone());
        instructions_1.push(instruction.clone());
    }

    let (tx0, rx1): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (tx1, rx0): (Sender<i64>, Receiver<i64>) = mpsc::channel();

    let thread0 = thread::spawn(move || {
        count_messages(&instructions_0, 0, tx0, rx0);
    });

    let thread1 = thread::spawn(move || {
        count_messages(&instructions_1, 1, tx1, rx1);
    });

    thread0.join().unwrap();
    thread1.join().unwrap();
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
            Instruction::Send(Value::FromRegister('a')),
            Instruction::Set('a', Value::Literal(0)),
            Instruction::Receive('a'),
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

    #[test]
    fn part_2_example() {
        let program = vec![
            Instruction::Send(Value::Literal(1)),
            Instruction::Send(Value::Literal(2)),
            Instruction::Send(Value::FromRegister('p')),
            Instruction::Receive('a'),
            Instruction::Receive('b'),
            Instruction::Receive('c'),
            Instruction::Receive('d')
        ];

        perform_duet(&program);
    }
}
