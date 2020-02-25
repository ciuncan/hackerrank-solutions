use std::collections::HashMap;

use crate::lib;

/// Solution for:
/// https://www.hackerrank.com/challenges/sock-merchant/problem

#[allow(dead_code)]
pub fn main() {
    let mut line = lib::read_std_line(Some("Number of colors should be entered!"));

    let n = line
        .trim()
        .parse::<usize>()
        .expect("First line should have number of colors only!");

    lib::read_std_line_into(&mut line, Some("Colors should be entered!"));

    let numbers = lib::parse_separated::<u32>(&line, n);

    let counts = count_colors(&numbers);
    let pair_count = find_pair_count(&counts);
    println!("{:}", pair_count);
}

fn count_colors(colors: &[u32]) -> HashMap<u32, usize> {
    let mut map = HashMap::new();
    for color in colors {
        let current_count = map.entry(*color).or_default();
        *current_count += 1;
    }
    map
}

fn find_pair_count(color_counts: &HashMap<u32, usize>) -> u32 {
    color_counts.values().map(|v| (v / 2) as u32).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib;

    #[test]
    fn check_100_numbers_input() {
        let input = "44 55 11 15 4 72 26 91 80 14 43 78 70 75 36 83 78 91 17 17 54 65 60 21 58 98 87 45 75 97 81 18 51 43 84 54 66 10 44 45 23 38 22 44 65 9 78 42 100 94 58 5 11 69 26 20 19 64 64 93 60 96 10 10 39 94 15 4 3 10 1 77 48 74 20 12 83 97 5 82 43 15 86 5 35 63 24 53 27 87 45 38 34 7 48 24 100 14 80 54";
        let numbers = lib::parse_separated::<u32>(&input, 100);
        let a = count_colors(&numbers);
        let pair_count = find_pair_count(&a);

        println!("{:?}", a);
        println!("{:?}", pair_count);
        assert_eq!(30, pair_count);
    }
}
