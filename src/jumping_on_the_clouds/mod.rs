use crate::lib;

#[allow(dead_code)]
pub fn main() {
    let mut line = lib::read_std_line(None);
    let n = line.parse::<usize>().unwrap();

    lib::read_std_line_into(&mut line, None);
    let clouds: Vec<Cloud> = lib::parse_separated(&line, n);

    let total_jumps = calculate_total_cumps(&clouds);

    println!("{:}", total_jumps);
}

enum Cloud {
    Cumulus,
    Thunderhead,
}

use crate::enum_per_char;

enum_per_char! { Cloud,
    default => Cumulus,
    '0' => Cumulus,
    '1' => Thunderhead
}

fn calculate_total_cumps(clouds: &[Cloud]) -> usize {
    let run_lengths = cumulus_run_lengths(&clouds);
    run_lengths
        .iter()
        .enumerate()
        .map(|(idx, l)| min_jump_count(*l, idx == run_lengths.len() - 1))
        .sum()
}

fn cumulus_run_lengths(clouds: &[Cloud]) -> Vec<usize> {
    let mut lengths = Vec::with_capacity(clouds.len());
    let mut count = 0;
    for cloud in clouds {
        match cloud {
            Cloud::Cumulus => count += 1,
            Cloud::Thunderhead => {
                lengths.push(count);
                count = 0;
            }
        }
    }
    lengths.push(count);
    lengths
}

fn min_jump_count(run_length: usize, is_last: bool) -> usize {
    if is_last {
        run_length / 2
    } else {
        let run_length = run_length + 1;
        run_length / 2 + run_length % 2
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cumulus_run_length() {
        let clouds = vec![
            Cloud::Cumulus,
            Cloud::Thunderhead,
            Cloud::Cumulus,
            Cloud::Cumulus,
            Cloud::Cumulus,
            Cloud::Thunderhead,
            Cloud::Cumulus,
        ];
        let expected = vec![1, 3, 1];
        assert_eq!(expected, cumulus_run_lengths(&clouds));
    }

    #[test]
    fn min_jump_count_test() {
        assert_eq!(0, min_jump_count(1, true));
        assert_eq!(1, min_jump_count(1, false));
        assert_eq!(2, min_jump_count(2, false));
        assert_eq!(2, min_jump_count(3, false));
        assert_eq!(3, min_jump_count(4, false));
        assert_eq!(3, min_jump_count(5, false));
    }

    #[test]
    fn min_jump_required_for() {
        let run = vec![
            Cloud::Cumulus,
            Cloud::Cumulus,
            Cloud::Thunderhead,
            Cloud::Cumulus,
            Cloud::Cumulus,
            Cloud::Thunderhead,
            Cloud::Cumulus,
        ];

        let actual = calculate_total_cumps(&run);
        let expected = 4;

        assert_eq!(expected, actual);
    }

    #[test]
    fn failed_test_case() {
        // 0 0 0 1 0 0
        let run = vec![
            Cloud::Cumulus,
            Cloud::Cumulus,
            Cloud::Cumulus,
            Cloud::Thunderhead,
            Cloud::Cumulus,
            Cloud::Cumulus,
        ];
        let expected = 3;
        let actual = calculate_total_cumps(&run);
        assert_eq!(expected, actual);
    }
}
