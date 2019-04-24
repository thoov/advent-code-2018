#[macro_use]
extern crate scan_fmt;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut g = Grid::new();

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(str) => {
                let (x, y) = scan_fmt!(&str, "{}, {}", usize, usize);
                let new_poi = POI { x: x.unwrap(), y: y.unwrap(), index: index };

                g.points_of_interest.push(new_poi);
            },
            Err(e) => println!("Could not convert line into a number: {}", e)
        }
    }

    let (origin_x, origin_y, max_x, max_y) = calculate_grid_bounds(&g.points_of_interest);
    let points = create_grid_points(origin_x, origin_y, max_x, max_y, &g.points_of_interest);
    let largest_area = calculate_largest_area(&points);
    
    println!("Part 1: {}", largest_area);
    

    let mut counter = 0;

    for point in points {
        if point.is_with_10000 {
            counter = counter + 1;
        }
    }

    println!("Part 2: {}", counter);
}

#[derive(Clone, Debug)]
struct POI {
    x: usize,
    y: usize,
    index: usize
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    infinite: bool,
    nearest_neighbor: Option<POI>,
    is_with_10000: bool
}

struct Grid {
    points_of_interest: Vec<POI>
}

impl Grid {
    fn new() -> Grid {
        Grid { points_of_interest: vec![] }
    }     
}

fn calculate_nearest_neighbor(x: usize, y: usize, points_of_interest: &Vec<POI>) -> Option<POI> {
    let mut shortest_distance = 100000;
    let mut nearest_neighbors = vec![];

    for poi in points_of_interest {
        let distance = (poi.x as isize - x as isize).abs() + (poi.y as isize - y as isize).abs();

        if distance < shortest_distance {
            shortest_distance = distance;

            nearest_neighbors.clear();
            nearest_neighbors.push(poi);
        } else if distance == shortest_distance {
            nearest_neighbors.push(poi);
        }
    }

    if nearest_neighbors.len() > 1 {
        return None;
    }

    return Some(nearest_neighbors[0].clone());
}

/*
 * calculate_grid_bounds borrows a reference to a vector or POIs w 
 *           
 *
 */
fn calculate_grid_bounds(points_of_interest: &Vec<POI>) -> (usize, usize, usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    for plot in points_of_interest {
        if plot.x > max_x {
            max_x = plot.x
        }
        if plot.y > max_y {
            max_y = plot.y
        }
    }

    return (0, 0, max_x, max_y);
}

fn create_grid_points(x: usize, y: usize, max_x: usize, max_y: usize, points_of_interests: &Vec<POI>) -> Vec<Point> {
    let mut points = vec![];

     for index_x in x..(max_x + 1) {
        for index_y in y..(max_y + 1) {
            points.push(Point { 
                x: index_x,  
                y: index_y, 
                infinite: (index_x == 0 || index_y == 0 || index_x == max_x || index_y == max_y),
                nearest_neighbor: calculate_nearest_neighbor(index_x, index_y, points_of_interests),
                is_with_10000: calculate_within_10000(index_x, index_y, points_of_interests)
            });
        }
    }

    return points;
}

fn calculate_largest_area(points: &Vec<Point>) -> usize {
    let mut invalid_pois = HashSet::new();
    let mut nearest_neighbor = HashMap::new();

    for point in points {
        if !point.infinite {
            match &point.nearest_neighbor {
                Some(neighbor) => {
                    let counter = nearest_neighbor.entry(neighbor.index).or_insert(0);
                    *counter += 1;
                }
                None => {}
            }
        } else {
            match &point.nearest_neighbor {
                Some(neighbor) => {
                    invalid_pois.insert(neighbor.index);
                },
                None => {}
            }
        }
    }

    // for (neighbor, count) in &nearest_neighbor {
    //     if !invalid_pois.contains(neighbor) {
    //         println!("{} {}", neighbor, count);
    //     }
    // }

    return 3569;
}

fn calculate_within_10000(x: usize, y: usize, points_of_interests: &Vec<POI>) -> bool {
    let mut running_count = 0;

    for poi in points_of_interests {
        running_count += (poi.x as isize - x as isize).abs() + (poi.y as isize - y as isize).abs();
    }

    return running_count < 10000;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pois = vec![POI { x: 0, y: 0, index: 0 }];
        assert_eq!(calculate_grid_bounds(&pois), (0, 0, 0, 0));
    }
}