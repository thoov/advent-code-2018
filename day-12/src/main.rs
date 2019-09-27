#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone, Debug)]
enum PlantState {
    EMPTY,
    FILLED
}

#[derive(PartialEq, Debug, Clone)]
struct Plant {
    position: i64,
    state: PlantState
}

#[derive(Debug, PartialEq)]
struct Note {
    next_state: PlantState,
    current: PlantState,
    left_left_side: PlantState,
    left_side: PlantState,
    right_right_side: PlantState,
    right_side: PlantState
}

struct Change {
    index: usize,
    next_state: PlantState
}

fn main() {
    let generations: i64 = 20;
    let mut plants = load_plants_from_str("#...#..###.#.###.####.####.#..#.##..#..##..#.....#.#.#.##.#...###.#..##..#.##..###..#..##.#..##...", 0);

    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut notes = vec![];

    for line in reader.lines() {
        match line {
            Ok(str) => { notes.push(create_note(&str))},
            Err(e) => println!("Could not convert line: {}", e),
        }
    }

    let mut previous_total = 0;
    for _i in 0..generations {
        next_generation(&notes, &mut plants);

        let total = calculate_count(&plants);
        println!("Gen: {} Total: {} Diff: {}", _i + 1, total, total - previous_total);
        previous_total = total;
    }

    println!("Final Total: {}", calculate_count(&plants));
}

fn char_to_state(plant_state: char) -> PlantState {
    match plant_state {
        '.' => return PlantState::EMPTY,
        _ => return PlantState::FILLED
    }
}

fn create_note(note: &str) -> Note {
  let (
      left_left_side,
      left_side,
      current,
      right_side,
      right_right_side,
      next_state
    ) = scan_fmt_some!(note, "{/./}{/./}{/./}{/./}{/./} => {/./}", char, char, char, char, char, char);

   return Note {
    current: char_to_state(current.unwrap()),
    next_state: char_to_state(next_state.unwrap()),
    left_left_side: char_to_state(left_left_side.unwrap()),
    left_side: char_to_state(left_side.unwrap()),
    right_side: char_to_state(right_side.unwrap()),
    right_right_side: char_to_state(right_right_side.unwrap())
   };
}

fn create_boundry_plant(position: i64) -> Plant {
    return Plant {
        position,
        state: PlantState::EMPTY
    };
}

fn next_generation(notes: &Vec<Note>, current_gen: &mut VecDeque<Plant>) {
    let mut changes: HashMap<usize, Change> = HashMap::new();

    for (i, plant) in current_gen.iter().enumerate() {
        let ll_side = if i < 2 { PlantState::EMPTY } else { current_gen.get(i - 2).unwrap().state };
        let l_side = if i < 1 { PlantState::EMPTY } else { current_gen.get(i - 1).unwrap().state };
        let r_side = if i > current_gen.len() - 2 { PlantState::EMPTY } else { current_gen.get(i + 1).unwrap().state };
        let rr_side = if i > current_gen.len() - 3 { PlantState::EMPTY } else { current_gen.get(i + 2).unwrap().state };

        for note in notes.iter() {
            if plant.state == note.current {
                if ll_side == note.left_left_side && 
                    l_side == note.left_side &&
                    r_side == note.right_side &&
                    rr_side == note.right_right_side {
                    changes.insert(i, Change { index: i, next_state: note.next_state });
                }
            }
        }
    }

    for (_i, change) in changes.iter() {
        current_gen.get_mut(change.index).unwrap().state = change.next_state;
    }

    current_gen.push_front(create_boundry_plant(current_gen.front().unwrap().position - 1));
    current_gen.push_back(create_boundry_plant(current_gen.back().unwrap().position + 1));
}

fn load_plants_from_str(initial_state: &str, start_idx: i64) -> VecDeque<Plant> {
    let mut plants: VecDeque<Plant> = VecDeque::new();

    // because we have to check 2 plants on each side we need to add empty plants
    let buffer_plants = 2;

    for i in 0..buffer_plants {
        plants.push_back(Plant { position: i - buffer_plants + start_idx, state: PlantState::EMPTY });
    }

    for (i, char) in initial_state.chars().enumerate() {
        let index = i as i64 + start_idx;
        match char {
            '.' => plants.push_back(Plant { position: index, state: PlantState::EMPTY }),
            '#' => plants.push_back(Plant { position: index, state: PlantState::FILLED }),
            _ => {}
        }
    }

    for i in 0..buffer_plants {
        plants.push_back(Plant { position: i + initial_state.len() as i64 + start_idx, state: PlantState::EMPTY });
    }

    return plants;
}

fn calculate_count(plants: &VecDeque<Plant>) -> i64 {
    let mut running_count = 0;

    for plant in plants.iter() {
        if plant.state == PlantState::FILLED {
            running_count += plant.position;
        }
    }

    return running_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_plants_from_str_works() {
        assert_eq!(load_plants_from_str(".#.", 0), VecDeque::from(vec![
            Plant { position: -2, state: PlantState::EMPTY },
            Plant { position: -1, state: PlantState::EMPTY },
            Plant { position: 0, state: PlantState::EMPTY },
            Plant { position: 1, state: PlantState::FILLED },
            Plant { position: 2, state: PlantState::EMPTY },
            Plant { position: 3, state: PlantState::EMPTY },
            Plant { position: 4, state: PlantState::EMPTY }
        ]));
    }

    #[test]
    fn calculate_count_works() {
        let plants = load_plants_from_str("...##....#####...#######....#.#..##.", 0);
        assert_eq!(calculate_count(&plants), 327);

        let plants = load_plants_from_str(".", 0);
        assert_eq!(calculate_count(&plants), 0);

        let plants = load_plants_from_str("##", -1);
        assert_eq!(calculate_count(&plants), -1);
    }

    #[test]
    fn create_note_works() {
        assert_eq!(create_note("...#. => #"), Note {
            current: PlantState::EMPTY,
            next_state: PlantState::FILLED,
            left_left_side: PlantState::EMPTY,
            left_side: PlantState::EMPTY,
            right_side: PlantState::FILLED,
            right_right_side: PlantState::EMPTY
        });
    }

    #[test]
    fn char_to_state_works() {
        assert_eq!(char_to_state('.'), PlantState::EMPTY);
        assert_eq!(char_to_state('#'), PlantState::FILLED);
    }
}
