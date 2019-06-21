use std::collections::VecDeque;

// Took ideas from: https://gist.github.com/tpoliaw/fcf797ea99ed183635717f02258f1c9b

trait Circle<T> {
    fn clockwise(&mut self, i: usize);
    fn counter_clockwise(&mut self, i: usize);
}

impl<T> Circle<T> for VecDeque<T> {
    fn clockwise(&mut self, i : usize) {
        for _ in 0..i {
            if let Some(val) = self.pop_back() {
                self.push_front(val);
            }
        }
    }

    fn counter_clockwise(&mut self, i : usize) {
        for _ in 0..i {
            if let Some(val) = self.pop_front() {
                self.push_back(val);
            }
        }
    }
}

fn play_game(total_players: usize, total_marbles: usize) -> usize {
    let mut circle: VecDeque<usize> = VecDeque::new();
    let mut scores = vec![0 as usize; total_players];

    // manually put the first marble onto the circle
    circle.push_back(0);

    for turn in 1..=total_marbles {
        if turn % 23 == 0 {
            circle.clockwise(7);

            scores[turn % total_players] += turn;

            if let Some(val) = circle.pop_back() {
                scores[turn % total_players] += val;
            }

            circle.counter_clockwise(1);
        } else {
            circle.counter_clockwise(1);
            circle.push_back(turn);
        }
    }

    return *scores.iter().max().unwrap_or(&0);
}

fn main() {
    println!("Part 1: {:?}", play_game(424, 71482));
    println!("Part 2: {:?}", play_game(424, 7148200));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_works() {
        assert_eq!(play_game(13, 7999), 146373);
        assert_eq!(play_game(17, 1104), 2764);
        assert_eq!(play_game(21, 6111), 54718);
        assert_eq!(play_game(30, 5807), 37305);

        // Part 1
        assert_eq!(play_game(424, 71482), 408679);

        // Part 2
        assert_eq!(play_game(424, 7148200), 3443939356);
    }
}
