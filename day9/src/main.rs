#[macro_use]
extern crate failure;

use std::fmt;
use std::str;
use failure::Error;

#[derive(Debug, PartialEq)]
struct TreeNode {
    depth: u32,
    garbage: u32,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(input: &str) -> Result<TreeNode, Error> {
        let mut node = TreeNode {
            depth: 1,
            garbage: 0,
            children: Vec::new(),
        };
        let mut chars = input.chars();
        ensure!(chars.next() == Some('{'), "Input didn't begin with '{'.");
        node.parse(&mut chars)?;
        ensure!(chars.next() == None, "Didn't parse entire input.");
        Ok(node)
    }

    fn parse<'a>(&mut self, chars: &mut str::Chars<'a>) -> Result<(), Error> {
        while let Some(c) = chars.next() {
            match c {
                '{' => {
                    let mut child = TreeNode {
                        depth: self.depth + 1,
                        garbage: 0,
                        children: Vec::new(),
                    };
                    child.parse(chars)?;
                    self.children.push(child);
                }
                '}' => return Ok(()),
                '!' => {
                    chars.next();
                }
                '<' => loop {
                    match chars.next() {
                        Some('!') => {
                            chars.next();
                        }
                        Some('>') => break,
                        Some(_) => {
                            self.garbage += 1;
                        }
                        None => bail!("Didn't find closing angle bracket."),
                    };
                },
                ',' => continue,
                _ => bail!("Unexpected character hit."),
            };
        }
        Err(format_err!("Didn't find closing curly brace."))
    }

    fn score(&self) -> u32 {
        self.depth + self.children.iter().map(|c| c.score()).sum::<u32>()
    }

    fn garbage_score(&self) -> u32 {
        self.garbage + self.children.iter().map(|c| c.garbage_score()).sum::<u32>()
    }

    fn fmt_helper<T: std::fmt::Write>(&self, f: &mut T, num_tabs: u32) -> fmt::Result {
        let tabs = std::iter::repeat("  ")
            .take(num_tabs as usize)
            .collect::<String>();
        writeln!(f, "{}TreeNode {{", tabs)?;
        writeln!(f, "{}  depth: {}", tabs, self.depth)?;
        writeln!(f, "{}  garbage: {}", tabs, self.garbage)?;
        if self.children.is_empty() {
            writeln!(f, "{}  children: []", tabs)?;
        } else {
            writeln!(f, "{}  children: [", tabs)?;
            for child in self.children.iter() {
                child.fmt_helper(f, num_tabs + 2)?;
                writeln!(f)?;
            }
            writeln!(f, "{}  ]", tabs)?;
        }
        write!(f, "{}}}", tabs)
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_helper(f, 0)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();
    println!("input:\n{}\n", input);
    let root = TreeNode::new(input).expect("Failed to read.");
    println!("result 1: {}", root.score());
    println!("result 2: {}", root.garbage_score());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_test() {
        assert_eq!(TreeNode::new("{}").unwrap().score(), 1);
        assert_eq!(TreeNode::new("{{{}}}").unwrap().score(), 6);
        assert_eq!(TreeNode::new("{{},{}}").unwrap().score(), 5);
        assert_eq!(TreeNode::new("{{{},{},{{}}}}").unwrap().score(), 16);
        assert_eq!(TreeNode::new("{<a>,<a>,<a>,<a>}").unwrap().score(), 1);
        assert_eq!(
            TreeNode::new("{{<ab>},{<ab>},{<ab>},{<ab>}}")
                .unwrap()
                .score(),
            9
        );
        assert_eq!(
            TreeNode::new("{{<!!>},{<!!>},{<!!>},{<!!>}}")
                .unwrap()
                .score(),
            9
        );
        assert_eq!(
            TreeNode::new("{{<a!>},{<a!>},{<a!>},{<ab>}}")
                .unwrap()
                .score(),
            3
        );
    }

    #[test]
    fn garbage_score_test() {
        assert_eq!(TreeNode::new("{<>}").unwrap().garbage_score(), 0);
        assert_eq!(
            TreeNode::new("{<random characters>}")
                .unwrap()
                .garbage_score(),
            17
        );
        assert_eq!(TreeNode::new("{<<<<>}").unwrap().garbage_score(), 3);
        assert_eq!(TreeNode::new("{<{!>}>}").unwrap().garbage_score(), 2);
        assert_eq!(TreeNode::new("{<!!>}").unwrap().garbage_score(), 0);
        assert_eq!(
            TreeNode::new("{<{o\"i!a,<{i<a>}").unwrap().garbage_score(),
            10
        );
    }
}
