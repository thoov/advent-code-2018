use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut numbers = vec![];

    for line in reader.lines() {
        match line {
            Ok(str) => {
                numbers = str.split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
            },
            Err(e) => println!("Could not convert line into a number: {}", e)
        }
    }

    let node = get_node_from_list(numbers.as_slice());
    println!("Part 1: {}", calc_metadata(&node));
    println!("Part 2: {}", calc_metadata_v2(&node));
}

fn calc_metadata(node: &Node) -> usize {
    let mut count = 0;

    for i in 0..node.metadata.len() {
        count += node.metadata[i];
    }

    for i in 0..node.children.len() {
        count += calc_metadata(&node.children[i]);
    }

    return count;
}

fn calc_metadata_v2(node: &Node) -> usize {
    let mut count = 0;

    if node.children.len() == 0 {
        for i in 0..node.metadata.len() {
            count += node.metadata[i];
        }

        return count;
    }

    for i in 0..node.metadata.len() {
        if node.metadata[i] <= node.children.len() && node.metadata[i] > 0 {
            count += calc_metadata_v2(&node.children[node.metadata[i] - 1]);
        }      
    }

    return count;
}

fn calc_offset_from_node(node: &Node) -> usize {
    let mut offset = 2 + node.header.1;

    for i in 0..node.children.len() {
        offset += calc_offset_from_node(&node.children[i]);
    }

    return offset;
}

fn get_node_from_list(list: &[usize]) -> Node {
    let num_of_children = list[0];
    let num_of_metadata = list[1];
    let header_offset = 2;

    let mut parent_node = Node {
        header: (num_of_children, num_of_metadata),
        children: vec![],
        metadata: vec![]
    };

    // base condition
    if num_of_children == 0 {
        parent_node.metadata = list[header_offset..header_offset + num_of_metadata].to_vec();
        return parent_node;
    }

    // we need to deal with children
    let mut children_list = list[header_offset..].to_vec();
    for _i in 0..num_of_children {
        let child_node = get_node_from_list(&children_list);
        let child_offset = calc_offset_from_node(&child_node);

        children_list = children_list[child_offset..children_list.len()].to_vec();
        parent_node.children.push(child_node);
    }

    parent_node.metadata = children_list[..num_of_metadata].to_vec();

    return parent_node;
}

#[derive(Debug)]
struct Node {
    header: (usize, usize),
    metadata: Vec<usize>,
    children: Vec<Node>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let example = &[2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let node = get_node_from_list(example);

        assert_eq!(node.header, (2, 3));
        assert_eq!(node.metadata, [1, 1, 2]);

        assert_eq!(node.children[0].header, (0, 3));
        assert_eq!(node.children[0].metadata, [10, 11, 12]);

        assert_eq!(node.children[1].header, (1, 1));
        assert_eq!(node.children[1].metadata, [2]);

        assert_eq!(node.children[1].children[0].header, (0, 1));
        assert_eq!(node.children[1].children[0].metadata, [99]);

        assert_eq!(calc_metadata(&node), 138);
        assert_eq!(calc_metadata_v2(&node), 66);
    }

    #[test]
    fn multiple_children_with_works() {
        let example = &[2, 1, 0, 1, 7, 0, 1, 8, 9];
        let node = get_node_from_list(example);

        assert_eq!(node.header, (2, 1));
        assert_eq!(node.metadata, [9]);
        assert_eq!(node.children.len(), 2);

        let child = &node.children[0];
        assert_eq!(child.header, (0, 1));
        assert_eq!(child.metadata, [7]);
        assert_eq!(child.children.len(), 0);

        let child = &node.children[1];
        assert_eq!(child.header, (0, 1));
        assert_eq!(child.metadata, [8]);
        assert_eq!(child.children.len(), 0);

        assert_eq!(calc_metadata(&node), 24);
    }

    #[test]
    fn base_case_works() {
        let example = &[0, 0];
        let node = get_node_from_list(example);
        assert_eq!(node.header, (0, 0));
        assert_eq!(node.metadata.len(), 0);
        assert_eq!(node.children.len(), 0);

        assert_eq!(calc_metadata(&node), 0);
        
        let example = &[0, 2, 4, 5];
        let node = get_node_from_list(example);
        assert_eq!(node.header, (0, 2));
        assert_eq!(node.metadata, [4, 5]);
        assert_eq!(node.children.len(), 0);

        assert_eq!(calc_metadata(&node), 9);
        assert_eq!(calc_metadata_v2(&node), 9);
    }

    #[test]
    fn one_layer_deep_works() {
        let example = &[1, 1, 0, 1, 7, 8];
        let node = get_node_from_list(example);

        assert_eq!(node.header, (1, 1));
        assert_eq!(node.metadata, [8]);
        assert_eq!(node.children.len(), 1);

        let child = &node.children[0];
        assert_eq!(child.header, (0, 1));
        assert_eq!(child.metadata, [7]);
        assert_eq!(child.children.len(), 0);
    }

    #[test]
    fn no_metadata_works() {
        let example = &[1, 1, 0, 0, 1];
        let node = get_node_from_list(example);

        assert_eq!(node.header, (1, 1));
        assert_eq!(node.metadata, [1]);
        assert_eq!(node.children.len(), 1);

        let child = &node.children[0];
        assert_eq!(child.header, (0, 0));
        assert_eq!(child.metadata, []);
        assert_eq!(child.children.len(), 0);

        assert_eq!(calc_metadata(&node), 1);
    }

    #[test]
    fn n_layers_works() {
        /*
        2  1  1  1  0  1  6  7  0  1  8  9
        A---------------------------------
              B---------------- D------ 
                    C------

        */
        let example = &[2, 1, 1, 1, 0, 1, 6, 7, 0, 1, 8, 9];
        let node = get_node_from_list(example);

        assert_eq!(node.header, (2, 1));
        assert_eq!(node.metadata, [9]);
        assert_eq!(node.children.len(), 2);

        let child = &node.children[0];
        assert_eq!(child.header, (1, 1));
        assert_eq!(child.metadata, [7]);
        assert_eq!(child.children.len(), 1);

        let child = &node.children[0].children[0];
        assert_eq!(child.header, (0, 1));
        assert_eq!(child.metadata, [6]);
        assert_eq!(child.children.len(), 0);

        let child = &node.children[1];
        assert_eq!(child.header, (0, 1));
        assert_eq!(child.metadata, [8]);
        assert_eq!(child.children.len(), 0);

        assert_eq!(calc_metadata(&node), 30);
        assert_eq!(calc_metadata_v2(&node), 0);
    }

    #[test]
    fn v2_works() {
        let example = &[2, 1, 1, 1, 0, 1, 1, 1, 0, 1, 8, 1];
        let node = get_node_from_list(example);
        assert_eq!(calc_metadata_v2(&node), 1);

        let example = &[1, 1, 0, 1, 7, 1];
        let node = get_node_from_list(example);
        assert_eq!(calc_metadata_v2(&node), 7);
    }
}
