#[macro_use]
extern crate scan_fmt;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut graph = Graph::new();
    let mut graph_2 = Graph::new();

    for line in reader.lines() {
        match line {
            Ok(str) => {
                let (finish_first, finish_second) = scan_fmt!(&str, "Step {} must be finished before step {} can begin.", char, char);
                graph.add_node_pair(finish_first.unwrap(), finish_second.unwrap(), 0);
                graph_2.add_node_pair(finish_first.unwrap(), finish_second.unwrap(), 60);
            },
            Err(e) => println!("Could not convert line into a number: {}", e)
        }
    }

    println!("Part 1: {}", graph.get_sequence()); // AEMNPOJWISZCDFUKBXQTHVLGRY
    println!("Part 2: {}", graph_2.get_completion_time(5)); // 1081
}

struct Node {
    parents: Vec<char>,
    children: Vec<char>,
    effort: usize
}

struct Graph {
    nodes: HashMap<char, Node>
}

impl Graph {
    fn new() -> Graph {
        Graph { nodes: HashMap::new() }
    }

    fn add_node_pair(&mut self, parent: char, child: char, offset: usize) {
        let parent_node = self.nodes.entry(parent).or_insert(Node {
            parents: vec![],
            children: vec![],
            effort: get_effort(parent, offset)
        });
        
        if parent_node.children.contains(&child) {
            println!("{} {}", parent, child);
        }

        parent_node.children.push(child);
        parent_node.children.sort_by(|a, b| a.cmp(b));

        let child_node = self.nodes.entry(child).or_insert(Node {
            parents: vec![],
            children: vec![],
            effort: get_effort(child, offset)
        });
        
        child_node.parents.push(parent);
        child_node.parents.sort_by(|a, b| a.cmp(b));
    }

    fn get_free_nodes(&self) -> Vec<char> {
        let mut entry_points = vec![];

        for (key, value) in self.nodes.iter() {
            if value.parents.len() == 0 {
                entry_points.push(*key);
            }
        }

        entry_points.sort_by(|a, b| a.cmp(b));
        return entry_points;
    }

    fn remove_parent_from_nodes(&mut self, parent: char) {
        for (_, value) in self.nodes.iter_mut() {
            if value.parents.contains(&parent) {
                let index = value.parents.iter().position(|x| *x == parent).unwrap();
                value.parents.remove(index);
            }
        }

        self.nodes.remove(&parent);
    }

    fn get_sequence(&mut self) -> String {
        let mut free_nodes = self.get_free_nodes();
        let mut final_sequence: Vec<char> = vec![];

        while free_nodes.len() > 0 {
            self.remove_parent_from_nodes(free_nodes[0]);

            final_sequence.push(free_nodes[0]);

            free_nodes = self.get_free_nodes();
        }

        return final_sequence.into_iter().collect();
    }

    fn reduce_effort(&mut self, number_of_workers: usize) -> bool {
        let free_nodes = self.get_free_nodes();

        if free_nodes.len() == 0 {
            return false;
        }
        
        for x in 0..number_of_workers {
            if x < free_nodes.len() {
                let node = self.nodes.get_mut(&free_nodes[x]).unwrap();
                node.effort -= 1;

                if node.effort == 0 {
                    self.remove_parent_from_nodes(free_nodes[x]);
                }
            }
        }

        return true;
    }

    fn get_completion_time(&mut self, workers: usize) -> usize {
        let mut time = 0;

        while self.reduce_effort(workers) {
            time += 1;
        }

        return time;
    }
}

fn get_effort(c: char, mut offset: usize) -> usize {
    match c {
        'a' | 'A' => { offset += 1 },
        'b' | 'B' => { offset += 2 },
        'c' | 'C' => { offset += 3 },
        'd' | 'D' => { offset += 4 },
        'e' | 'E' => { offset += 5 },
        'f' | 'F' => { offset += 6 },
        'g' | 'G' => { offset += 7 },
        'h' | 'H' => { offset += 8 },
        'i' | 'I' => { offset += 9 },
        'j' | 'J' => { offset += 10 },
        'k' | 'K' => { offset += 11 },
        'l' | 'L' => { offset += 12 },
        'm' | 'M' => { offset += 13 },
        'n' | 'N' => { offset += 14 },
        'o' | 'O' => { offset += 15 },
        'p' | 'P' => { offset += 16 },
        'q' | 'Q' => { offset += 17 },
        'r' | 'R' => { offset += 18 },
        's' | 'S' => { offset += 19 },
        't' | 'T' => { offset += 20 },
        'u' | 'U' => { offset += 21 },
        'v' | 'V' => { offset += 22 },
        'w' | 'W' => { offset += 23 },
        'x' | 'X' => { offset += 24 },
        'y' | 'Y' => { offset += 25 },
        'z' | 'Z' => { offset += 26 },
        _ => { offset += 0 }
    }

    return offset;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_free_nodes_works() {
        let mut graph = Graph::new();
        graph.add_node_pair('c', 'a', 0);
        graph.add_node_pair('c', 'f', 0);
        graph.add_node_pair('a', 'b', 0);
        graph.add_node_pair('a', 'd', 0);
        graph.add_node_pair('b', 'e', 0);
        graph.add_node_pair('d', 'e', 0);
        graph.add_node_pair('f', 'e', 0);
        assert_eq!(graph.get_free_nodes(), vec!['c']);

        graph.remove_parent_from_nodes('c');
        assert_eq!(graph.get_free_nodes(), vec!['a', 'f']);
    }
    
    #[test]
    fn get_sequence_works() {
        let mut graph = Graph::new();

        graph.add_node_pair('c', 'a', 0);
        graph.add_node_pair('c', 'f', 0);
        graph.add_node_pair('a', 'b', 0);
        graph.add_node_pair('a', 'd', 0);
        graph.add_node_pair('b', 'e', 0);
        graph.add_node_pair('d', 'e', 0);
        graph.add_node_pair('f', 'e', 0);

        assert_eq!(graph.get_sequence(), String::from("cabdfe"));
    }

    #[test]
    fn get_completion_time_works() {
        let mut graph = Graph::new();

        graph.add_node_pair('c', 'a', 0);
        graph.add_node_pair('c', 'f', 0);
        graph.add_node_pair('a', 'b', 0);
        graph.add_node_pair('a', 'd', 0);
        graph.add_node_pair('b', 'e', 0);
        graph.add_node_pair('d', 'e', 0);
        graph.add_node_pair('f', 'e', 0);

        assert_eq!(graph.get_completion_time(2), 16); // There might be a bug in this
    }
}
