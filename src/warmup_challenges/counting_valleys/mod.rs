use crate::lib;

/// Solution for:
/// https://www.hackerrank.com/challenges/counting-valleys/problem

#[allow(dead_code)]
pub fn main() {
    let steps: Vec<Dir> = lib::parse_two_line_input();

    let walk = Walk::new(StepState::walk(steps));
    let valley_count = walk.valley_count();

    println!("{:}", valley_count);
}

struct Walk(Vec<StepState>);

impl Walk {
    fn new(steps: Vec<StepState>) -> Walk {
        Walk(steps)
    }

    fn valley_count(&self) -> usize {
        self.0.iter().filter(|state| state.new_valley()).count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StepState {
    last_dir: Dir,
    dir: Dir,
    current_level: i32,
}

impl StepState {
    fn new(first_dir: Dir, second_dir: Dir) -> StepState {
        StepState {
            last_dir: first_dir,
            dir: second_dir,
            current_level: first_dir.level_diff() + second_dir.level_diff(),
        }
    }

    fn new_valley(self) -> bool {
        self.current_level == -1 && self.dir == Dir::D
    }

    fn step_mut(&mut self, next_dir: Dir) {
        self.last_dir = self.dir;
        self.dir = next_dir;
        self.current_level += next_dir.level_diff();
    }

    fn walk(directions: Vec<Dir>) -> Vec<StepState> {
        if directions.len() < 2 {
            vec![]
        } else {
            let first_dir = directions[0];
            let second_dir = directions[1];

            let initial_state = StepState::new(first_dir, second_dir);
            let mut walks = Vec::with_capacity(directions.len());
            walks.push(initial_state.clone());
            walks.extend(
                directions
                    .iter()
                    .skip(2)
                    .scan(initial_state, |last_state, next_dir| {
                        last_state.step_mut(*next_dir);
                        Some(*last_state)
                    }),
            );
            walks
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    U,
    D,
}

impl Dir {
    pub fn level_diff(self) -> i32 {
        match self {
            Dir::U => 1,
            Dir::D => -1,
        }
    }
}

use crate::enum_per_char;

enum_per_char! { Dir,
    default => U,
    'U' => U,
    'D' => D
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn walk_without_steps_produce_empty() {
        let empty_steps: Vec<Dir> = vec![];
        let result = StepState::walk(empty_steps);
        assert_eq!(Vec::<StepState>::new(), result);
    }

    #[test]
    fn walk_with_less_than_2_steps_produce_empty() {
        let singleton_steps = vec![Dir::U];
        let result = StepState::walk(singleton_steps);
        assert_eq!(Vec::<StepState>::new(), result);
    }

    #[test]
    fn walk_with_2_steps_should_create_initial_state() {
        let singleton_steps = vec![Dir::U, Dir::D];
        let states = StepState::walk(singleton_steps);

        let expected = vec![StepState {
            last_dir: Dir::U,
            dir: Dir::D,
            current_level: 0,
        }];

        assert_eq!(expected, states);
    }

    macro_rules! mk_steps {
        () => {
            Vec::<Dir>::empty()
        };
        ($x:ident, $($y:ident),+) => {
            vec![Dir::$x, $(Dir::$y),+]
        }
    }

    #[test]
    fn walk_with_series_of_directions() {
        let directions = mk_steps!(U, D, D, D, U, D, U, U);
        let expected = vec![
            StepState {
                last_dir: Dir::U,
                dir: Dir::D,
                current_level: 0,
            },
            StepState {
                last_dir: Dir::D,
                dir: Dir::D,
                current_level: -1,
            },
            StepState {
                last_dir: Dir::D,
                dir: Dir::D,
                current_level: -2,
            },
            StepState {
                last_dir: Dir::D,
                dir: Dir::U,
                current_level: -1,
            },
            StepState {
                last_dir: Dir::U,
                dir: Dir::D,
                current_level: -2,
            },
            StepState {
                last_dir: Dir::D,
                dir: Dir::U,
                current_level: -1,
            },
            StepState {
                last_dir: Dir::U,
                dir: Dir::U,
                current_level: 0,
            },
        ];
        let actual = StepState::walk(directions);
        assert_eq!(expected, actual);
    }

    #[test]
    fn walk_valley_count_single() {
        let directions = Walk(StepState::walk(mk_steps!(U, D, D, D, U, D, U, U)));
        assert_eq!(1, directions.valley_count());
    }
}
