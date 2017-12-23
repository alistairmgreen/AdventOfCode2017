pub mod errors;
pub mod instructions;
pub use instructions::{Instruction, Value};
use std::collections::HashMap;

pub type Register = char;

pub struct Processor {
    registers: HashMap<Register, i64>,
    multiplications: usize,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            registers: HashMap::new(),
            multiplications: 0,
        }
    }

    pub fn multiplication_count(&self) -> usize {
        self.multiplications
    }

    pub fn execute(&mut self, instructions: &[Instruction]) {
        let mut index: usize = 0;
        let instruction_count = instructions.len();

        while index < instruction_count {
            match instructions[index] {
                Instruction::Set(ref register, ref value) => {
                    let x = self.get_value(&value);
                    *self.get_mut(&register) = x;
                }
                Instruction::Subtract(ref register, ref value) => {
                    let x = self.get_value(&value);
                    *self.get_mut(&register) -= x;
                }
                Instruction::Multiply(ref register, ref value) => {
                    self.multiplications += 1;
                    let x = self.get_value(&value);
                    *self.get_mut(&register) *= x;
                }
                Instruction::JumpIfNotZero(_, _) => {}
            }

            index = match instructions[index] {
                Instruction::JumpIfNotZero(ref condition, ref value) => {
                    if self.get_value(condition) == 0 {
                        index + 1
                    } else {
                        ((index as i64) + self.get_value(value)) as usize
                    }
                }
                _ => index + 1,
            }
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
