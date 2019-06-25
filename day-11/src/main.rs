fn main() {
    let serial_number = 3463;
    let mut grid = vec![vec![0 as i32; 300]; 300];

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let rack_id = x as i32 + 10;
            let starting_power_level = rack_id * y as i32;
            let increased_power_level = starting_power_level + serial_number;
            let power_level = increased_power_level * rack_id;

            let vec = number_to_vec(power_level);
            let mut hundreds_digit:i32 = 0;

            if vec.len() > 2 {
                hundreds_digit = vec[vec.len() - 3] as i32;
            }

            let final_power_level = hundreds_digit - 5;
            grid[x][y] = final_power_level as i32;
        }
    }

    // find largest section
    // let mut highest_power = vec![0 as i32; 300];
    // let mut highest_coordinate = vec![(0, 0); 300];

    for k in 1..=12 {
        let mut highest_power = -100000;
        let mut highest_coordinate = (0, 0);

        for x in 0..grid.len() - (k - 1) {
            for y in 0..grid[x].len() - (k - 1) {

                let power_total = power_of_sub_grid((x, y), k, &grid);
                if power_total > highest_power {
                    highest_power = power_total;
                    highest_coordinate = (x, y);
                }
            }
        }

        println!("Size: {} Power: {} Coordinates: {:?}", k, highest_power, highest_coordinate);
    }

    // println!("Part 1: {:?}", highest_coordinate[3]); // (235, 60) power 28
    // let index = highest_power.iter().enumerate().map(|(x, y)| (y, x)).max().unwrap().1;
    // println!("Part 2: {:?}", (highest_coordinate[index], index));
}

fn power_of_sub_grid(offset: (usize, usize), sub_grid_size: usize, grid: &Vec<Vec<i32>>) -> i32 {
    let mut power_total = 0;

    for x in offset.0..(offset.0 + sub_grid_size) {
        for y in offset.1..(offset.1 + sub_grid_size) {
            power_total += grid[x][y];
        }
    }

    return power_total;
}

fn number_to_vec(n: i32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
