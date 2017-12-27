#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;
use failure::Error;

#[derive(Clone)]
enum Value {
    Register(char),
    Immediate(i64),
}

#[derive(Clone)]
enum Instruction {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
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
                do_parse!(tag!("snd") >> x: ws!(value) >> (Instruction::Snd(x)))
                    | do_parse!(
                        tag!("set") >> x: ws!(register) >> y: ws!(value)
                            >> (Instruction::Set(x, y))
                    )
                    | do_parse!(
                        tag!("add") >> x: ws!(register) >> y: ws!(value)
                            >> (Instruction::Add(x, y))
                    )
                    | do_parse!(
                        tag!("mul") >> x: ws!(register) >> y: ws!(value)
                            >> (Instruction::Mul(x, y))
                    )
                    | do_parse!(
                        tag!("mod") >> x: ws!(register) >> y: ws!(value)
                            >> (Instruction::Mod(x, y))
                    )
                    | do_parse!(tag!("rcv") >> x: ws!(register) >> (Instruction::Rcv(x)))
                    | do_parse!(
                        tag!("jgz") >> x: ws!(value) >> y: ws!(value) >> (Instruction::Jgz(x, y))
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
    Waiting(char),
    Terminated,
}

#[derive(Clone)]
struct Program {
    registers: Registers,
    imem: Vec<Instruction>,
    pc: isize,
    state: ProgramState,
    send_queue: VecDeque<i64>,
}

impl Program {
    fn new(imem: Vec<Instruction>, pid: i64) -> Program {
        let mut p = Program {
            registers: HashMap::new(),
            imem: imem,
            pc: 0,
            state: ProgramState::Runnable,
            send_queue: VecDeque::new(),
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

    fn send(&mut self) -> Option<i64> {
        self.send_queue.pop_front()
    }

    fn recieve(&mut self, val: i64) {
        if let ProgramState::Waiting(reg) = self.state {
            self.registers.insert(reg, val);
            self.state = ProgramState::Runnable;
        } else {
            panic!("Called recieve when in state: {:?}", self.state);
        }
    }

    fn state<'a>(&'a self) -> &'a ProgramState {
        &self.state
    }

    fn run_cycle(&mut self) {
        assert_eq!(self.state, ProgramState::Runnable);

        if let Some(instr) = self.imem.get(self.pc as usize) {
            use Instruction::*;
            match instr {
                &Snd(ref x) => {
                    let x = self.val(x);
                    self.send_queue.push_back(x);
                }
                &Set(x, ref y) => {
                    let y = self.val(y);
                    self.registers.insert(x, y);
                }
                &Add(reg, ref y) => {
                    let x = self.reg(reg);
                    let y = self.val(y);
                    self.registers.insert(reg, x + y);
                }
                &Mul(reg, ref y) => {
                    let x = self.reg(reg);
                    let y = self.val(y);
                    self.registers.insert(reg, x * y);
                }
                &Mod(reg, ref y) => {
                    let x = self.reg(reg);
                    let y = self.val(y);
                    self.registers.insert(reg, x % y);
                }
                &Rcv(reg) => {
                    self.state = ProgramState::Waiting(reg);
                }
                &Jgz(ref x, ref y) => {
                    if self.val(x) > 0 {
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

struct Cpu {
    prog1: Program,
    prog2: Program,
}

impl Cpu {
    fn new(imem: Vec<Instruction>) -> Cpu {
        Cpu {
            prog1: Program::new(imem.clone(), 0),
            prog2: Program::new(imem, 0),
        }
    }

    fn run(&mut self) -> usize {
        let mut count = 0;
        while self.prog1.state() == &ProgramState::Runnable
            || self.prog2.state() == &ProgramState::Runnable
        {
            match self.prog1.state() {
                &ProgramState::Runnable => self.prog1.run_cycle(),
                &ProgramState::Waiting(_) => {
                    if let Some(val) = self.prog2.send() {
                        count += 1;
                        self.prog1.recieve(val);
                    }
                }
                &ProgramState::Terminated => (),
            }
            match self.prog2.state() {
                &ProgramState::Runnable => self.prog2.run_cycle(),
                &ProgramState::Waiting(_) => {
                    if let Some(val) = self.prog1.send() {
                        self.prog2.recieve(val);
                    }
                }
                &ProgramState::Terminated => (),
            }
        }

        count + self.prog2.send_queue.len()
    }
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, Error> {
    input
        .split('\n')
        .map(str::parse::<Instruction>)
        .collect::<Result<Vec<_>, Error>>()
}

fn last_send(imem: Vec<Instruction>) -> Result<i64, Error> {
    let mut program = Program::new(imem, 0);
    while program.state() == &ProgramState::Runnable {
        program.run_cycle();
    }
    if let Some(&val) = program.send_queue.back() {
        Ok(val)
    } else {
        Err(format_err!("Never sent anything."))
    }
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();

    let instructions = parse_input(&input).expect("parse");
    let result = last_send(instructions.clone()).expect("fail");
    println!("Result 1: {}", result);

    let mut cpu = Cpu::new(instructions);
    let result = cpu.run();
    println!("Result 2: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_send() {
        let input = "set a 1\n\
                     add a 2\n\
                     mul a a\n\
                     mod a 5\n\
                     snd a\n\
                     set a 0\n\
                     rcv a\n\
                     jgz a -1\n\
                     set a 1\n\
                     jgz a -2";
        let instructions = parse_input(&input).unwrap();
        let result = last_send(instructions).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_cpu_run() {
        let input = "snd 1\n\
                     snd 2\n\
                     snd p\n\
                     rcv a\n\
                     rcv b\n\
                     rcv c\n\
                     rcv d";
        let instructions = parse_input(&input).unwrap();
        let mut cpu = Cpu::new(instructions);
        assert_eq!(cpu.run(), 3);
    }

    #[test]
    fn test_cpu_run_2() {
        let input = "set a 1\n\
                     add a 2\n\
                     mul a a\n\
                     mod a 5\n\
                     snd a\n\
                     set a 0\n\
                     rcv a\n\
                     jgz a -1\n\
                     set a 1\n\
                     jgz a -2";
        let instructions = parse_input(&input).unwrap();
        let mut cpu = Cpu::new(instructions);
        assert_eq!(cpu.run(), 1);
    }
}
