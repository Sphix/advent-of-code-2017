#![feature(iterator_step_by)]
#![feature(inclusive_range_syntax)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::str::FromStr;
use failure::Error;

#[derive(Clone)]
enum Value {
    Register(char),
    Immediate(i64),
}

#[derive(Clone)]
enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(register<&str, char>, one_of!("abcdefghijklmnopqrstuvwxyz"));

        named!(integer<&str, i64>, map!(
                pair!(
                    map!(opt!(tag!("-")),
                        |sign| if sign.is_some() { -1i64 } else { 1i64 }),
                    map_res!(nom::digit, str::parse::<i64>)),
                |(sign, val)| sign * val));

        named!(value<&str, Value>, alt!(
                integer => {|n| Value::Immediate(n) } |
                register => { |r| Value::Register(r) }
            ));

        complete!(
            s,
            alt!(
                    do_parse!(
                        tag!("set") >> x: ws!(register) >> y: ws!(value)
                            >> (Instruction::Set(x, y))
                    )
                    | do_parse!(
                        tag!("sub") >> x: ws!(register) >> y: ws!(value)
                            >> (Instruction::Sub(x, y))
                    )
                    | do_parse!(
                        tag!("mul") >> x: ws!(register) >> y: ws!(value)
                            >> (Instruction::Mul(x, y))
                    )
                    | do_parse!(
                        tag!("jnz") >> x: ws!(value) >> y: ws!(value) >> (Instruction::Jnz(x, y))
                    )
            )
        ).to_result()
            .map_err(|e| format_err!("{}", e))
    }
}

type Registers = HashMap<char, i64>;

#[derive(Clone, Debug, PartialEq)]
enum ProgramState {
    Runnable,
    Terminated,
}

#[derive(Clone)]
struct Core<'a> {
    registers: Registers,
    imem: &'a [Instruction],
    pc: isize,
    state: ProgramState,
    multiply_count: usize,
}

impl<'a> Core<'a> {
    fn new(imem: &'a [Instruction], pid: i64) -> Core {
        let mut p = Core {
            registers: HashMap::new(),
            imem: imem,
            pc: 0,
            state: ProgramState::Runnable,
            multiply_count: 0,
        };
        p.registers.insert('p', pid);
        p
    }

    fn reg(&self, reg: char) -> i64 {
        *self.registers.get(&reg).unwrap_or(&0)
    }

    fn val(&self, val: &Value) -> i64 {
        match val {
            &Value::Register(r) => self.reg(r),
            &Value::Immediate(i) => i,
        }
    }

    fn state<'b>(&'b self) -> &'b ProgramState {
        &self.state
    }

    fn run_cycle(&mut self) {
        assert_eq!(self.state, ProgramState::Runnable);

        if let Some(instr) = self.imem.get(self.pc as usize) {
            use Instruction::*;
            match instr {
                &Set(x, ref y) => {
                    let y = self.val(y);
                    self.registers.insert(x, y);
                }
                &Sub(reg, ref y) => {
                    let x = self.reg(reg);
                    let y = self.val(y);
                    self.registers.insert(reg, x - y);
                }
                &Mul(reg, ref y) => {
                    self.multiply_count += 1;
                    let x = self.reg(reg);
                    let y = self.val(y);
                    self.registers.insert(reg, x * y);
                }
                &Jnz(ref x, ref y) => {
                    if self.val(x) != 0 {
                        self.pc += self.val(y) as isize - 1;
                    }
                }
            }
            self.pc += 1;
        } else {
            self.state = ProgramState::Terminated;
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, Error> {
    input
        .split('\n')
        .map(str::parse::<Instruction>)
        .collect::<Result<Vec<_>, Error>>()
}

fn multiply_count(imem: &[Instruction]) -> usize {
    let mut program = Core::new(imem, 0);
    while program.state() == &ProgramState::Runnable {
        program.run_cycle();
    }
    program.multiply_count
}

fn run_program() -> usize {
    // This is the reverse engineered source code.
    (108_400..=125_400).step_by(17).filter(|val| {
        for i in 2..=((val / 2)) {
            if val % i == 0 {
                return true;
            }
        }
        return false;
    }).count()
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();

    let instructions = parse_input(&input).expect("parse");
    let result = multiply_count(&instructions);
    println!("Result 1: {}", result);

    let result = run_program();
    println!("Result 2: {}", result);
}
