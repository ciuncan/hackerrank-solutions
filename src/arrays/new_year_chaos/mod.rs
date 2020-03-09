use crate::lib;

/// Solution for:
/// https://www.hackerrank.com/challenges/new-year-chaos/problem

#[allow(dead_code)]
pub fn main() {
    let t = lib::read_std_line(None).parse::<usize>().unwrap();
    for _ in 0..t {
        let mut order = lib::parse_two_line_input();
        for i in order.iter_mut() {
            *i -= 1;
        }
        let state = QueueState::new(order);
        match state.min_bribe_count() {
            None => {
                println!("Too chaotic");
            }
            Some(bribe_count) => {
                println!("{:}", bribe_count);
            }
        };
    }
}

#[derive(Clone, Debug)]
struct QueueState {
    order: Vec<usize>,
    bribe_count: usize,
}

impl QueueState {
    fn new(order: Vec<usize>) -> QueueState {
        QueueState {
            order,
            bribe_count: 0,
        }
    }

    fn min_bribe_count(mut self) -> Option<usize> {
        if self.order.is_empty() {
            return Some(0);
        }
        let mut index = self.order.len().wrapping_sub(1);
        let mut count = 0;

        while index > 0 {
            let mut actual_index = index;
            while index != self.order[actual_index] {
                actual_index -= 1;
                if index - actual_index > 2 {
                    return None;
                }
            }

            let current_count = index - actual_index;
            count += current_count;

            while actual_index < index {
                self.order.swap(actual_index, actual_index + 1);
                actual_index += 1;
            }

            index -= 1;
        }

        Some(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_bribe_count_ordered() {
        let state = QueueState::new((0..5).collect());
        assert_eq!(Some(0), state.min_bribe_count());
    }

    #[test]
    fn min_bribe_count_empty() {
        let state = QueueState::new(vec![0; 0]);
        assert_eq!(Some(0), state.min_bribe_count());
    }

    #[test]
    fn min_bribe_count_single_bribe() {
        let state = QueueState::new(vec![0, 2, 1, 3, 4]);
        assert_eq!(Some(1), state.min_bribe_count());
    }

    #[test]
    fn min_bribe_count_too_chaotic() {
        let state = QueueState::new(vec![1, 4, 0, 2, 3]);
        assert_eq!(None, state.min_bribe_count());
    }

    #[test]
    fn min_bribe_count_sample() {
        let state = QueueState::new(vec![1, 0, 4, 2, 3]);
        assert_eq!(Some(3), state.min_bribe_count());
    }
}
