use std::collections::HashMap;

pub type Register = char;

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    FromRegister(Register),
    Literal(i32),
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

struct Registers {
    registers: HashMap<Register, i32>,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            registers: HashMap::new(),
        }
    }

    fn get_value(&self, value: &Value) -> i32 {
        match *value {
            Value::FromRegister(r) => *self.registers.get(&r).unwrap_or(&0),
            Value::Literal(v) => v,
        }
    }

    fn get_mut(&mut self, register: &Register) -> &mut i32 {
        self.registers.entry(register.clone()).or_insert(0)
    }
}

pub fn play(instructions: &[Instruction]) -> Option<i32> {
    let mut registers = Registers::new();
    let mut last_sound: Option<i32> = None;
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
            _ => 1
        };

        index = (index as i32 + increment) as usize;
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
}