#[macro_use]
extern crate failure;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Value {
    Zero,
    One,
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Value::Zero),
            "1" => Ok(Value::One),
            _ => Err(format_err!("Invalid"))
        }
    }
}

#[derive(Debug)]
enum Movement {
    Left,
    Right,
}

impl FromStr for Movement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Movement::Left),
            "right" => Ok(Movement::Right),
            _ => Err(format_err!("Invalid"))
        }
    }
}

#[derive(Debug)]
struct SubState {
    write: Value,
    movement: Movement,
    next_state: char,
}

#[derive(Debug)]
struct State {
    zero: SubState,
    one: SubState,
}

#[derive(Debug)]
struct States(HashMap<char, State>);

impl FromStr for States {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state_re = Regex::new(r"In state ([A-Z]):").unwrap();
        let sub_state_re = Regex::new(r"If the current value is (0|1):").unwrap();
        let write_re = Regex::new(r"- Write the value (0|1).").unwrap();
        let movement_re = Regex::new(r"- Move one slot to the (right|left).").unwrap();
        let next_state_re = Regex::new(r"- Continue with state ([A-Z]).").unwrap();

        let mut states = HashMap::new();
        let mut iter = s.split('\n');
        while let Some(line) = iter.next() {
            if let Some(caps) = state_re.captures(line) {
                ensure!(
                    &sub_state_re.captures(iter.next().unwrap()).unwrap()[1] == "0",
                    "what"
                );
                let zero = SubState {
                    write: write_re
                        .captures(iter.next().unwrap())
                        .ok_or(format_err!("write"))?[1]
                        .parse::<Value>()?,
                    movement: movement_re
                        .captures(iter.next().unwrap())
                        .ok_or(format_err!("movement"))?[1]
                        .parse::<Movement>()?,
                    next_state: next_state_re
                        .captures(iter.next().unwrap())
                        .ok_or(format_err!("next state"))?[1]
                        .chars()
                        .nth(0)
                        .unwrap(),
                };
                ensure!(
                    &sub_state_re.captures(iter.next().unwrap()).unwrap()[1] == "1",
                    "what"
                );
                let one = SubState {
                    write: write_re
                        .captures(iter.next().unwrap())
                        .ok_or(format_err!("write"))?[1]
                        .parse::<Value>()?,
                    movement: movement_re
                        .captures(iter.next().unwrap())
                        .ok_or(format_err!("movement"))?[1]
                        .parse::<Movement>()?,
                    next_state: next_state_re
                        .captures(iter.next().unwrap())
                        .ok_or(format_err!("next state"))?[1]
                        .chars()
                        .nth(0)
                        .unwrap(),
                };
                states.insert(
                    caps[1].chars().nth(0).unwrap(),
                    State {
                        zero: zero,
                        one: one,
                    },
                );
            }
        }
        Ok(States(states))
    }
}

struct TuringMachine {
    states: States,
    tape: VecDeque<Value>,
    current_state: char,
    current_index: usize,
}

impl TuringMachine {
    fn new(states: States, starting_state: char) -> TuringMachine {
        TuringMachine {
            states: states,
            tape: [Value::Zero].into_iter().cloned().collect(),
            current_state: starting_state,
            current_index: 0,
        }
    }
    fn step(&mut self) {
        let state = match self.tape[self.current_index] {
            Value::Zero => {
                &self.states.0[&self.current_state].zero
            }
            Value::One => {
                &self.states.0[&self.current_state].one
            }
        };
        self.tape[self.current_index] = state.write;
        match state.movement {
            Movement::Left => {
                if self.current_index == 0 {
                    self.tape.push_front(Value::Zero);
                } else {
                    self.current_index -= 1;
                }
            }
            Movement::Right => {
                self.current_index += 1;
                if self.current_index == self.tape.len() {
                    self.tape.push_back(Value::Zero);
                }
            }
        };
        self.current_state = state.next_state;
    }

    fn checksum(&self) -> usize {
        self.tape.iter().filter(|&val| *val == Value::One).count()
    }
}

fn main() {
    let input = include_str!("input.txt");
    let states: States = input.parse().expect("parse");
    let mut machine = TuringMachine::new(states, 'A');
    (0..12_586_542).for_each(|_| machine.step());

    println!("Result 1: {}", machine.checksum());
}
