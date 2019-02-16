use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let original_polymer = reader.lines().nth(0).unwrap().unwrap();

    println!("Original polymer length: {}", original_polymer.len());

    let second_copy = original_polymer.clone();
    let part_1_polymer = reduce_polymer(original_polymer);

    println!("Part 1 length: {}", part_1_polymer.len());

    let units = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut shortest_polymer = part_1_polymer.len(); // must be shorter than doing "nothing"
    for unit in units {
        let cloned = second_copy
            .clone()
            .replace(unit, "")
            .replace(unit.to_ascii_uppercase(), "");
        let p = reduce_polymer(cloned);

        if p.len() < shortest_polymer {
            shortest_polymer = p.len();
        }
    }

    println!("Shortest Length: {}", shortest_polymer);
}

fn reduce_polymer(mut polymer: String) -> String {
    let mut found_index: isize = 0;

    while found_index >= 0 {
        found_index = find_index_of_pair(&polymer);
        if found_index >= 0 {
            polymer.replace_range(found_index as usize..(found_index as usize) + 2, "");
        }
    }

    return polymer;
}

fn find_index_of_pair(poly: &str) -> isize {
    let mut last_char = '0';
    let mut found_index: isize = -1;

    for (index, c) in poly.chars().enumerate() {
        let uppercase = c.to_ascii_uppercase();
        let lowercase = c.to_ascii_lowercase();

        if uppercase == last_char && c != uppercase {
            found_index = (index as isize) - 1;
            break;
        } else if lowercase == last_char && c != lowercase {
            found_index = (index as isize) - 1;
            break;
        }

        last_char = c;
    }

    return found_index;
}
