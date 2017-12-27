#[macro_use]
extern crate lazy_static;
extern crate regex;
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Cond {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

impl FromStr for Cond {
    type Err = &'static str;
    fn from_str(cond: &str) -> Result<Self, Self::Err> {
        match cond {
            ">" => Ok(Cond::GreaterThan),
            ">=" => Ok(Cond::GreaterThanOrEqual),
            "<" => Ok(Cond::LessThan),
            "<=" => Ok(Cond::LessThanOrEqual),
            "==" => Ok(Cond::Equal),
            "!=" => Ok(Cond::NotEqual),
            _ => Err("Invalid condition"),
        }
    }
}

impl Cond {
    fn compare<T: Ord>(self, lhs: T, rhs: T) -> bool {
        match self {
            Cond::GreaterThan => lhs > rhs,
            Cond::GreaterThanOrEqual => lhs >= rhs,
            Cond::LessThan => lhs < rhs,
            Cond::LessThanOrEqual => lhs <= rhs,
            Cond::Equal => lhs == rhs,
            Cond::NotEqual => lhs != rhs,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction<'a> {
    pub register: &'a str,
    pub amount: i32,
    pub cond: Cond,
    pub cond_register: &'a str,
    pub cond_amount: i32,
}

fn decode_instruction<'a>(line: &'a str) -> Option<Instruction<'a>> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(?P<register>[a-z]+)\\s+\
                                            (?P<op>inc|dec)\\s+\
                                            (?P<amount>-*\\d+)\\s+if\\s+\
                                            (?P<cond_register>[a-z]+)\\s+\
                                            (?P<cond>[<>=!]+)\\s+\
                                            (?P<cond_amount>-*\\d+)$").unwrap();
    }
    RE.captures(line).and_then(|caps| {
        let amount = str::parse::<i32>(&caps["amount"]).ok()?;
        let cond = str::parse::<Cond>(&caps["cond"]).ok()?;
        let cond_amount = str::parse::<i32>(&caps["cond_amount"]).ok()?;
        let amount = match &caps["op"] {
            "inc" => amount,
            "dec" => amount * -1,
            _ => unreachable!("bad regex"),
        };
        Some(Instruction {
            register: caps.name("register")?.as_str(),
            amount: amount,
            cond: cond,
            cond_register: caps.name("cond_register")?.as_str(),
            cond_amount: cond_amount,
        })
    })
}

// Returns maximum value in any register after running program.
fn run_program(input: &str) -> i32 {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    for (i, line) in input.split('\n').enumerate() {
        if let Some(instr) = decode_instruction(line.trim()) {
            registers.entry(&instr.cond_register).or_insert(0);
            let cond_reg = registers[&instr.cond_register];
            if instr.cond.compare(cond_reg, instr.cond_amount) {
                let reg = registers.entry(&instr.register).or_insert(0);
                *reg += instr.amount;
            }
        } else {
            println!("Unable to decode instruction on line {}:\n{}", i, line);
        }
    }
    *registers.values().max().unwrap()
}

fn run_program_2(input: &str) -> i32 {
    let mut registers = HashMap::new();
    let mut current_max = std::i32::MIN;
    for (i, line) in input.split('\n').enumerate() {
        if let Some(instr) = decode_instruction(line.trim()) {
            registers.entry(instr.cond_register).or_insert(0);
            let cond_reg = registers[instr.cond_register];
            if instr.cond.compare(cond_reg, instr.cond_amount) {
                let reg = registers.entry(instr.register).or_insert(0);
                *reg += instr.amount;
                current_max = std::cmp::max(current_max, *reg);
            }
        } else {
            println!("Unable to decode instruction on line {}:\n{}", i, line);
        }
    }
    current_max
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();
    println!("result 1: {}", run_program(input));
    println!("result 2: {}", run_program_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_instruction_test() {
        let expected = Some(Instruction {
            register: "b",
            amount: 5,
            cond: Cond::GreaterThan,
            cond_register: "a",
            cond_amount: 1,
        });
        assert_eq!(decode_instruction("b inc 5 if a > 1"), expected);
    }
    #[test]
    fn run_program_test() {
        let program = "b inc 5 if a > 1
                       a inc 1 if b < 5
                       c dec -10 if a >= 1
                       c inc -20 if c == 10";
        assert_eq!(run_program(program), 1);
    }
    #[test]
    fn run_program_2_test() {
        let program = "b inc 5 if a > 1
                       a inc 1 if b < 5
                       c dec -10 if a >= 1
                       c inc -20 if c == 10";
        assert_eq!(run_program_2(program), 10);
    }
}
