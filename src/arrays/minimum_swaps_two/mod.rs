use crate::lib;

/// Solution for:
/// https://www.hackerrank.com/challenges/minimum-swaps-2/problem

#[allow(dead_code)]
pub fn main() {
    let arr: Vec<usize> = lib::parse_two_line_input();
    let swap_count = minimum_swap_count(arr.iter().map(|a| a.wrapping_sub(1)).collect());
    println!("{}", swap_count);
}

fn minimum_swap_count(mut v: Vec<usize>) -> usize {
    let mut count = 0;
    for i in 0..v.len() {
        while v[i] != i {
            let actual_i = v[i];
            v.swap(i, actual_i);
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn minimum_swap_count_sample1() {
        let v: Vec<usize> = vec![0, 2, 4, 1, 3, 5, 6];
        let expected = 3;
        assert_eq!(expected, minimum_swap_count(v));
    }

    #[test]
    fn minimum_swap_count_sample2() {
        let v: Vec<usize> = vec![1, 2, 3, 0, 4];
        let expected = 3;
        assert_eq!(expected, minimum_swap_count(v));
    }
}
