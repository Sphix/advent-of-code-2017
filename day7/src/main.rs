extern crate regex;

use std::collections::HashMap;
use std::fmt;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
struct TreeNode {
    pub name: String,
    pub weight: u32,
    children: Vec<TreeNode>,
}

#[derive(Debug, PartialEq)]
enum Weight {
    Valid(u32),
    Invalid(u32),
}

impl TreeNode {
    fn new(name: &str, weight: u32) -> TreeNode {
        TreeNode {
            name: name.to_string(),
            weight: weight,
            children: Vec::new(),
        }
    }

    fn insert_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    // Either returns weight for itself and all children, or propogates invalid
    // weight when it is found.
    fn calc_weight(&self) -> Weight {
        if self.children.len() == 0 {
            return Weight::Valid(self.weight);
        }

        let mut weights = Vec::new();
        for weight in self.children.iter().map(|c| c.calc_weight()) {
            match weight {
                Weight::Valid(weight) => weights.push(weight),
                Weight::Invalid(_) => return weight,
            }
        }
        let max = weights.iter().max().unwrap();
        let min = weights.iter().min().unwrap();
        if max != min {
            let position = weights.iter().position(|n| n == max).unwrap();
            return Weight::Invalid(self.children[position].weight - (max - min));
        }

        Weight::Valid(self.weight + (max * weights.len() as u32))
    }

    fn fmt_helper<T: std::fmt::Write>(&self, f: &mut T, num_tabs: u32) -> fmt::Result {
        let tabs = std::iter::repeat("  ")
            .take(num_tabs as usize)
            .collect::<String>();
        writeln!(f, "{}TreeNode {{", tabs)?;
        writeln!(f, "{}  name: {}", tabs, self.name)?;
        writeln!(f, "{}  weight: {}", tabs, self.weight)?;
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

fn parse_input(input: &str) -> TreeNode {
    let root_re = Regex::new(r"^(?P<name>[a-z]+)\s+\((?P<weight>\d+)\)").unwrap();
    let child_re = Regex::new(r"^[a-z]+\s+\(\d+\)\s+->\s+(?P<children>[a-z, ]+)$").unwrap();

    let mut nodes = Vec::new();
    for line in input.split('\n') {
        if let Some(caps) = root_re.captures(line) {
            // Assuming regex engine already validated string is an integer.
            let weight = str::parse::<u32>(&caps["weight"]).unwrap();
            let node = TreeNode::new(&caps["name"], weight);
            if let Some(caps) = child_re.captures(line) {
                let children = caps["children"]
                    .split(", ")
                    .map(|s| (s.to_string(), true))
                    .collect::<HashMap<_, _>>();
                nodes.push((node, children));
            } else {
                nodes.push((node, HashMap::new()));
            }
        }
    }
    while nodes.len() > 1 {
        let (mut leaf_nodes, mut parent_nodes): (Vec<_>, Vec<_>) =
            nodes.into_iter().partition(|&(_, ref c)| c.is_empty());

        if leaf_nodes.is_empty() {
            panic!("Invalid input! Multiple roots!");
        }

        'outer: for (node, _) in leaf_nodes.drain(..) {
            for &mut (ref mut parent_node, ref mut children) in parent_nodes.iter_mut() {
                if children.remove(&node.name).is_some() {
                    parent_node.insert_child(node);
                    continue 'outer;
                }
            }
        }
        nodes = parent_nodes;
    }

    nodes.remove(0).0
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();
    println!("input:\n{}\n", input);
    let tree_root = parse_input(input);
    println!("result 1: {}", tree_root);
    println!("result 2: {:?}", tree_root.calc_weight());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let expected = TreeNode {
            name: "tknk".to_string(),
            weight: 41,
            children: vec![
                TreeNode {
                    name: "fwft".to_string(),
                    weight: 72,
                    children: vec![
                        TreeNode {
                            name: "xhth".to_string(),
                            weight: 57,
                            children: Vec::new(),
                        },
                        TreeNode {
                            name: "ktlj".to_string(),
                            weight: 57,
                            children: Vec::new(),
                        },
                        TreeNode {
                            name: "cntj".to_string(),
                            weight: 57,
                            children: Vec::new(),
                        },
                    ],
                },
                TreeNode {
                    name: "padx".to_string(),
                    weight: 45,
                    children: vec![
                        TreeNode {
                            name: "pbga".to_string(),
                            weight: 66,
                            children: Vec::new(),
                        },
                        TreeNode {
                            name: "havc".to_string(),
                            weight: 66,
                            children: Vec::new(),
                        },
                        TreeNode {
                            name: "qoyq".to_string(),
                            weight: 66,
                            children: Vec::new(),
                        },
                    ],
                },
                TreeNode {
                    name: "ugml".to_string(),
                    weight: 68,
                    children: vec![
                        TreeNode {
                            name: "ebii".to_string(),
                            weight: 61,
                            children: Vec::new(),
                        },
                        TreeNode {
                            name: "jptl".to_string(),
                            weight: 61,
                            children: Vec::new(),
                        },
                        TreeNode {
                            name: "gyxo".to_string(),
                            weight: 61,
                            children: Vec::new(),
                        },
                    ],
                },
            ],
        };

        assert_eq!(
            parse_input(
                "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"
            ),
            expected
        );
    }

    #[test]
    fn calc_weight_test() {
        let root = parse_input(
            "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
        );
        assert_eq!(root.calc_weight(), Weight::Invalid(60));
    }

    #[test]
    fn fmt_test() {
        let expected = "TreeNode {
  name: tknk
  weight: 41
  children: [
    TreeNode {
      name: fwft
      weight: 72
      children: [
        TreeNode {
          name: xhth
          weight: 57
          children: []
        }
        TreeNode {
          name: ktlj
          weight: 57
          children: []
        }
        TreeNode {
          name: cntj
          weight: 57
          children: []
        }
      ]
    }
    TreeNode {
      name: padx
      weight: 45
      children: [
        TreeNode {
          name: pbga
          weight: 66
          children: []
        }
        TreeNode {
          name: havc
          weight: 66
          children: []
        }
        TreeNode {
          name: qoyq
          weight: 66
          children: []
        }
      ]
    }
    TreeNode {
      name: ugml
      weight: 68
      children: [
        TreeNode {
          name: ebii
          weight: 61
          children: []
        }
        TreeNode {
          name: jptl
          weight: 61
          children: []
        }
        TreeNode {
          name: gyxo
          weight: 61
          children: []
        }
      ]
    }
  ]
}";

        let node = parse_input(
            "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
        );
        let mut s = String::new();
        let result = node.fmt_helper(&mut s, 0);
        assert_eq!(result, Ok(()));
        assert_eq!(&s, expected);
    }
}
