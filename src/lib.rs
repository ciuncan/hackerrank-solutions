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
    fn tapl(self, label: &str) -> Self {
        print!("{:}: ", label);
        self.tap()
    }
}

impl<T: Debug + Sized> Tappable<T> for T {
    fn tap(self) -> Self {
        println!("{:?}", self);
        self
    }
}

#[macro_export]
macro_rules! enum_per_char {
    ($tpe: ty, default => $def:ident, $($c:expr => $v:ident),+) => {

        impl std::str::FromStr for $tpe {
            type Err = ();

            fn from_str(s: &str) -> std::result::Result<Self, ()> {
                match s.chars().next() {
                    $(Some($c) => { Ok(<$tpe>::$v) }),+
                    _ => Err(()),
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
