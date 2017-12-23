pub mod errors;
pub mod instructions;
pub use instructions::{Instruction, Value};
use std::collections::{HashMap};

pub type Register = char;

pub struct Processor {
    registers: HashMap<Register, i64>,
    multiplications: usize,
}

impl Processor {
    pub fn debug() -> Processor {
        Processor {
            registers: HashMap::new(),
            multiplications: 0,
        }
    }

    pub fn release() -> Processor {
        let mut registers = HashMap::new();
        registers.insert('a', 1);
        Processor {
            registers: registers,
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
            Value::FromRegister(r) => self.get_register(r),
            Value::Literal(v) => v,
        }
    }

    fn get_mut(&mut self, register: &Register) -> &mut i64 {
        self.registers.entry(register.clone()).or_insert(0)
    }

    pub fn get_register(&self, register: Register) -> i64 {
        *self.registers.get(&register).unwrap_or(&0)
    }
}

pub fn primes_up_to(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();

    let mut candidate = 3;
    while candidate <= n {
        if !primes.iter().any(|n| candidate % n == 0) {
            primes.push(candidate);
        }

        candidate += 2;
    }

    primes.insert(0, 2);

    primes
}

#[cfg(test)]
mod tests {
    use super::primes_up_to;

    #[test]
    fn find_primes_up_to_20() {
        let primes = primes_up_to(20);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}