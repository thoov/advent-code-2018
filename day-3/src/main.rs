#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut fabric = HashMap::new();
    let mut all_claim_ids = vec![];

    for line in reader.lines() {
        match line {
            Ok(str) => {
                let (claim_id, left_offset, top_offset, width, height) =
                    convert_line_to_variables(str);
                let plots = dimensions_to_plots(left_offset, top_offset, width, height);
                all_claim_ids.push(claim_id);

                for plot in plots.iter() {
                    if !fabric.contains_key(&plot.to_string()) {
                        fabric.insert(plot.to_string(), vec![claim_id]);
                    } else {
                        let claims = fabric.get(plot);

                        match claims {
                            Some(c) => {
                                let mut claim = c.clone();
                                claim.push(claim_id);
                                fabric.insert(plot.to_string(), claim);
                            }
                            None => println!("Ooops"),
                        }
                    }
                }
            }
            Err(e) => println!("Could not convert line: {}", e),
        }
    }

    let mut count = 0;
    for val in fabric.values() {
        if val.len() > 1 {
            count += 1;

            for claim in val.iter() {
                all_claim_ids.iter()
                    .position(|&n| n == *claim)
                    .map(|e| all_claim_ids.remove(e))
                    .is_some();
            }
        }
    }

    println!("How many square inches of fabric are within two or more claims: {}", count);
    println!("What is the ID of the only claim that doesn't overlap: {:?}", all_claim_ids);
}

fn convert_line_to_variables(str: String) -> (u16, u16, u16, u16, u16) {
    // #1 @ 483,830: 24x18
    let (claim_id, left_offset, top_offset, width, height) =
        scan_fmt!(&str, "#{} @ {},{}: {}x{}", u16, u16, u16, u16, u16);

    return (
        claim_id.unwrap(),
        left_offset.unwrap(),
        top_offset.unwrap(),
        width.unwrap(),
        height.unwrap(),
    );
}

fn dimensions_to_plots(left_offset: u16, top_offset: u16, width: u16, height: u16) -> Vec<String> {
    let mut plots = vec![];

    for x in left_offset..(left_offset + width) {
        for y in top_offset..(top_offset + height) {
            plots.push(format!("{}x{}", x, y));
        }
    }

    return plots;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_line_to_variables() {
        assert_eq!(convert_line_to_variables(String::from("#1 @ 483,830: 24x18")), (1, 483, 830, 24, 18));
    }

    #[test]
    fn test_dimensions_to_plots() {
        assert_eq!(dimensions_to_plots(0, 0, 1, 2), vec!(String::from("0x0"), String::from("0x1")));
    }
}
