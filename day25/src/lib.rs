use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Default for State {
    fn default() -> State { State::A }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Bit {
    Zero,
    One,
}

impl Default for Bit {
    fn default() -> Bit { Bit::Zero }
}

#[derive(Debug, Default)]
pub struct TuringMachine {
    index: isize,
    state: State,
    tape: HashMap<isize, Bit>,
}

impl TuringMachine {
    pub fn new() -> TuringMachine {
        TuringMachine {
            index: 0,
            state: State::A,
            tape: HashMap::new(),
        }
    }

    pub fn step(&mut self) {
        let current_value = *self.tape.get(&self.index).unwrap_or(&Bit::Zero);

        self.state = match self.state {
            State::A => match current_value {
                Bit::Zero => {
                    self.tape.insert(self.index, Bit::One);
                    self.index += 1;
                    State::B
                }
                Bit::One => {
                    self.tape.insert(self.index, Bit::Zero);
                    self.index += 1;
                    State::C
                }
            },
            State::B => match current_value {
                Bit::Zero => {
                    self.index -= 1;
                    State::A
                }
                Bit::One => {
                    self.tape.insert(self.index, Bit::Zero);
                    self.index += 1;
                    State::D
                }
            },
            State::C => match current_value {
                Bit::Zero => {
                    self.tape.insert(self.index, Bit::One);
                    self.index += 1;
                    State::D
                }
                Bit::One => {
                    self.index += 1;
                    State::A
                }
            },
            State::D => match current_value {
                Bit::Zero => {
                    self.tape.insert(self.index, Bit::One);
                    self.index -= 1;
                    State::E
                }
                Bit::One => {
                    self.tape.insert(self.index, Bit::Zero);
                    self.index -= 1;
                    State::D
                }
            },
            State::E => match current_value {
                Bit::Zero => {
                    self.tape.insert(self.index, Bit::One);
                    self.index += 1;
                    State::F
                }
                Bit::One => {
                    self.index -= 1;
                    State::B
                }
            },
            State::F => match current_value {
                Bit::Zero => {
                    self.tape.insert(self.index, Bit::One);
                    self.index += 1;
                    State::A
                }
                Bit::One => {
                    self.index += 1;
                    State::E
                }
            },
        }
    }

    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    pub fn checksum(&self) -> usize {
        self.tape.values().filter(|&&v| v == Bit::One).count()
    }
}
