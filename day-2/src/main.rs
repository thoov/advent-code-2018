use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    println!("Part 1 checksum is: {}", part_1());
    println!("Part 2 correct box ids: {:?}", part_2());
}

fn part_1() -> i32 {
    let filename = "src/input.txt";
    let mut pairs = 0;
    let mut triplets = 0;


    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(str) => {
                let mut hash = HashMap::new();

                for id in str.chars() {
                    match hash.get(&id) {
                        Some(count) => hash.insert(id, count + 1),
                        _ => hash.insert(id, 1)
                    };
                }

                let mut has_pair = false;
                let mut has_triplet = false;

                for val in hash.values() {
                    if val == &2 {
                        has_pair = true;
                    } else if val == &3 {
                        has_triplet = true;
                    }
                }

                if has_pair {
                    pairs += 1;
                }
                if has_triplet {
                    triplets += 1;
                }
            }
            Err(e) => println!("Could not read line: {}", e)
        }
    }

    return pairs * triplets;
}

fn part_2() -> (String, String) {
    let filename = "src/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut box_ids: Vec<String> = vec![];
    let mut common_box_ids: (String, String) = (String::from("DID"), String::from("NOT WORK"));

    for line in reader.lines() {
        match line {
            Ok(str) => {
                for box_id in &box_ids {
                    let mut num_of_different_indexs = 0;
                    let mut current_chars = str.chars();

                    for char_a in box_id.chars() {
                        if char_a != current_chars.next().unwrap() {
                            num_of_different_indexs += 1;
                        }
                    }

                    if num_of_different_indexs <= 1 {
                        common_box_ids = (box_id.clone(), str.clone());
                    }
                }

                box_ids.push(str);
            }
            Err(e) => println!("Could not read line: {}", e)
        }
    }

    return common_box_ids;
}
