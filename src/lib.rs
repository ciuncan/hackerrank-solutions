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
