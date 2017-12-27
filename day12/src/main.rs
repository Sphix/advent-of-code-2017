#[macro_use]
extern crate failure;
extern crate petgraph;
extern crate regex;

use failure::Error;
use petgraph::graphmap::UnGraphMap;
use petgraph::visit::Bfs;
use petgraph::visit::Walker;
use regex::Regex;

fn parse_input(input: &str) -> Result<UnGraphMap<u32,()>, Error> {
    let re = Regex::new(r"^(?P<root>\d+) <-> (?P<neighbors>[\d, ]+)$")?;
    let mut graph = UnGraphMap::new();

    for caps in input.split('\n').map(|line| re.captures(line)) {
        ensure!(caps.is_some(), "Unable to parse line");
        let caps = caps.unwrap();
        let root: u32 = caps["root"].parse()?;
        if !graph.contains_node(root) {
            graph.add_node(root);
        }
        for neighbor in caps["neighbors"].split(", ") {
            let neighbor: u32 = neighbor.parse()?;
            if !graph.contains_node(neighbor) {
                graph.add_node(neighbor);
            }
            graph.add_edge(root, neighbor, ());
        }
    }
    Ok(graph)
}

fn neighbors_connected_to_root<E>(graph: &UnGraphMap<u32, E>) -> u32 {
    let bfs = Bfs::new(&graph, 0);
    bfs.iter(&graph).count() as u32
}

fn count_groups<E: Clone>(graph: &UnGraphMap<u32, E>) -> u32 {
    let mut count = 0;
    let mut graph_copy = graph.clone();
    loop {
        if let Some(root) = graph_copy.nodes().nth(0) {
            let bfs = Bfs::new(&graph, root);
            bfs.iter(&graph).for_each(|node| {
                graph_copy.remove_node(node);
            });
            count += 1;
        } else {
            return count;
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();
    let graph =  parse_input(input).expect("parse");

    let result = neighbors_connected_to_root(&graph);
    println!("Result 1: {}", result);

    let result = count_groups(&graph);
    println!("Result 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_connected_to_root_test() {
        let input = "0 <-> 2\n\
                     1 <-> 1\n\
                     2 <-> 0, 3, 4\n\
                     3 <-> 2, 4\n\
                     4 <-> 2, 3, 6\n\
                     5 <-> 6\n\
                     6 <-> 4, 5";
        let graph = parse_input(input).unwrap();
        assert_eq!(neighbors_connected_to_root(&graph), 6);
    }

    #[test]
    fn count_groups_test() {
        let input = "0 <-> 2\n\
                     1 <-> 1\n\
                     2 <-> 0, 3, 4\n\
                     3 <-> 2, 4\n\
                     4 <-> 2, 3, 6\n\
                     5 <-> 6\n\
                     6 <-> 4, 5";
        let graph = parse_input(input).unwrap();
        assert_eq!(count_groups(&graph), 2);
    }
}
