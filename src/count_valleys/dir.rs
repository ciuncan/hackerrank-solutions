use std::str::FromStr;

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

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, ()> {
        match s.chars().next() {
            Some('U') => Ok(Dir::U),
            Some('D') => Ok(Dir::D),
            _ => Err(()),
        }
    }
}

impl Default for Dir {
    fn default() -> Self {
        Dir::U
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_u_dir() {
        let dir_str = "U";
        assert_eq!(Ok(Dir::U), dir_str.parse::<Dir>());
    }

    #[test]
    fn parse_d_dir() {
        let dir_str = "D";
        assert_eq!(Ok(Dir::D), dir_str.parse::<Dir>());
    }
}
