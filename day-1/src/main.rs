use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let mut count: i64 = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(str) => count += convert_line_to_number(&str),
            Err(e) => println!("Could not convert line into a number: {}", e)
        }
    }

    println!("Final count is: {}", count);
}

/*
 * A line will be either:
 *   - positive integer: +5
 *   - negative integer: -450
 */
fn convert_line_to_number(line: &str) -> i64 {
    if line.contains("+") {
        return line[1..line.chars().count()].parse::<i64>().unwrap();
    }

    return line.parse::<i64>().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(convert_line_to_number("+4"), 4);
        assert_eq!(convert_line_to_number("-4"), -4);
        assert_eq!(convert_line_to_number("0"), 0);
    }
}
