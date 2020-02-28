use crate::lib;

#[allow(dead_code)]
pub fn main() {
    let mut line = lib::read_std_line(None);
    let base_str = line.clone();

    lib::read_std_line_into(&mut line, None);
    let n = line.parse::<usize>().unwrap();

    let a_count = count_a(&base_str, n);

    println!("{:}", a_count);
}

fn count_a(s: &str, n: usize) -> usize {
    if s.len() >= n {
        s.chars().take(n).filter(|c| *c == 'a').count()
    } else {
        let repeat_count = n / s.len();
        let existing = s.chars().filter(|c| *c == 'a').count();
        existing * repeat_count + count_a(s, n % s.len())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(7, count_a("aba", 10));
    }

    #[test]
    fn second_example() {
        assert_eq!(1_000_000_000_000, count_a("a", 1_000_000_000_000));
    }
}
