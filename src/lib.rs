use std::fmt::Debug;
use std::io::{stdin, BufRead};
use std::str::FromStr;

pub fn read_std_line(message: Option<&'static str>) -> String {
    let mut line = String::new();
    read_std_line_into(&mut line, message);
    line
}

pub fn read_std_line_into(line: &mut String, message: Option<&'static str>) {
    line.clear();
    stdin()
        .lock()
        .read_line(line)
        .unwrap_or_else(|_| panic!(message.unwrap_or("Line expected!")));

    if let Some('\n') = line.chars().last() {
        line.remove(line.len() - 1);
    }
}

pub fn read_matrix_from_std<I: FromStr + Default>(n: usize, m: usize) -> Vec<Vec<I>> {
    let mut lines = Vec::with_capacity(n);

    if n > 0 {
        let mut line = read_std_line(None);
        lines.push(parse_separated(&line, m));

        for _ in 1..n {
            read_std_line_into(&mut line, None);
            lines.push(parse_separated(&line, m));
        }
    }
    lines
}

pub fn parse_separated<N: FromStr + Default>(line: &str, at_most: usize) -> Vec<N> {
    line.split_whitespace()
        .take(at_most)
        .map(|s| s.parse::<N>().unwrap_or_default())
        .collect()
}

pub fn parse_two_line_input<N: FromStr + Default>() -> Vec<N> {
    let mut line = read_std_line(None);
    let n = line.parse::<usize>().unwrap();

    read_std_line_into(&mut line, None);
    parse_separated(&line, n)
}

pub trait Tappable<T: Sized>: Sized {
    fn tap(self) -> Self;
    fn tap_key(self, key: &str) -> Self {
        print!("{:}: ", key);
        self.tap()
    }
}

impl<T: Debug> Tappable<T> for &T {
    fn tap(self) -> Self {
        println!("{:?}", self);
        self
    }
}

impl<T: Debug> Tappable<T> for &mut T {
    fn tap(self) -> Self {
        println!("{:?}", self);
        self
    }
}

#[macro_export]
macro_rules! enum_per_char {
    (type = $tpe:ident; default => $def:ident; $($c:expr => $v:ident),+) => {

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $tpe {
            $( $v ),+
        }

        impl std::str::FromStr for $tpe {
            type Err = String;

            fn from_str(s: &str) -> std::result::Result<Self, String> {
                match s.chars().next() {
                    $(Some($c)  => { Ok(<$tpe>::$v) }),+
                    Some(c)     => Err(format!("Unknown character c={} for type $tpe", c)),
                    None        => Err(String::from("No more input left to parse type $tpe")),
                }
            }
        }

        impl Default for $tpe {
            fn default() -> Self {
                <$tpe>::$def
            }
        }

    };
}
