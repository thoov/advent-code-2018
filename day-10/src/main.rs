#[macro_use]
extern crate scan_fmt;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut points = vec![];

    for line in reader.lines() {
        match line {
            Ok(str) => {
                points.push(convert_line_to_variables(str))
            }
            Err(e) => println!("Could not convert line: {}", e),
        }
    }

    let mut seconds = 0;
    while seconds < 10400 {
        for point in points.iter_mut() {
            point.0 += point.2;
            point.1 += point.3;
        }

        seconds += 1;

        if points_spell_word(&points) {
            print_points(&points);
            println!("Part 2: {}", seconds);
        }
    }
}

fn print_points(points: &Vec<(i64, i64, i64, i64)>) {
    let min_x = points.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let min_y = points.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
    let max_x = points.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let max_y = points.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    for y in min_y - 3..=max_y + 3 {
        for x in min_x - 5..=max_x + 5 {
            if points.iter().find(|p| p.0 == x && p.1 == y) != None {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn points_are_connected(point_a: &(i64, i64, i64, i64), point_b: &(i64, i64, i64, i64)) -> bool {
    let x_diff = (point_a.0 - point_b.0).abs();
    let y_diff = (point_a.1 - point_b.1).abs();

    if x_diff < 2 && y_diff < 2 && (x_diff != 0 || y_diff != 0) {
        return true;
    }

    return false;
}

fn points_spell_word(points: &Vec<(i64, i64, i64, i64)>) -> bool {
    for point_a in points.iter() {
        let mut points_connected = false;
        for point_b in points.iter() {
            if points_are_connected(point_a, point_b) {
                points_connected = true;
            }
        }

        if !points_connected {
            return false;
        }
    }

    return true;
}

fn convert_line_to_variables(str: String) -> (i64, i64, i64, i64) {
    let (pos_x, pos_y, vel_x, vel_y) =
        scan_fmt!(&str, "position=<{},{}>velocity=<{},{}>", i64, i64, i64, i64);

    return (
        pos_x.unwrap(),
        pos_y.unwrap(),
        vel_x.unwrap(),
        vel_y.unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_line_to_variables_works() {
        assert_eq!(
            convert_line_to_variables(String::from("position=<-41214,-10223>velocity=<4,1>")),
            (-41214, -10223, 4, 1)
        );

        assert_eq!(
            convert_line_to_variables(String::from("position=< 10585,  41558> velocity=<-1, -4>")),
            (10585, 41558, -1, -4)
        );
    }
}
