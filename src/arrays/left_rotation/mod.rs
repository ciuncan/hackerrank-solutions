use crate::lib;

use std::fmt::Debug;
use std::ops::Index;

/// Solution for:
/// https://www.hackerrank.com/challenges/ctci-array-left-rotation/problem

#[allow(dead_code)]
pub fn main() {
    let mut line = lib::read_std_line(None);
    let size_and_rotations = lib::parse_separated::<usize>(&line, 2);
    let size = size_and_rotations[0];
    let left_rotations = size_and_rotations[1];

    lib::read_std_line_into(&mut line, None);
    let elems = lib::parse_separated::<i32>(&line, size);

    let rotated = RotatedArray::from_vec(left_rotations as isize, elems);

    println!("{:?}", rotated);
}

fn map_index(array_size: usize, left_rotations: isize, i: usize) -> usize {
    let mut remainder = (i as isize + left_rotations) % array_size as isize;
    if remainder < 0 {
        remainder += array_size as isize;
    }
    remainder as usize
}

struct RotatedArray<T> {
    left_rotations: isize,
    backing: Box<[T]>,
}

impl<T> RotatedArray<T> {
    fn from_vec(left_rotations: isize, vec: Vec<T>) -> RotatedArray<T> {
        RotatedArray {
            left_rotations,
            backing: vec.into_boxed_slice(),
        }
    }

    fn len(&self) -> usize {
        self.backing.len()
    }
}

impl<T> Index<usize> for RotatedArray<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.backing[map_index(self.backing.len(), self.left_rotations, idx)]
    }
}

struct RotatedArrayIter<'a, T> {
    curr_idx: usize,
    arr: &'a RotatedArray<T>,
}

impl<'a, T> Iterator for RotatedArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.curr_idx >= self.arr.len() {
            None
        } else {
            let result = Some(&self.arr[self.curr_idx]);
            self.curr_idx += 1;
            result
        }
    }
}

impl<'a, T> IntoIterator for &'a RotatedArray<T> {
    type Item = &'a T;
    type IntoIter = RotatedArrayIter<'a, T>;
    fn into_iter(self) -> RotatedArrayIter<'a, T> {
        RotatedArrayIter {
            curr_idx: 0,
            arr: self,
        }
    }
}

impl<T: Debug> Debug for RotatedArray<T> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        use std::fmt::Write;
        for v in self.into_iter() {
            v.fmt(formatter)?;
            formatter.write_char(' ')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_case() {
        let given: Vec<i32> = vec![1, 2, 3, 4, 5];
        let left_rotations = 4;

        let expected: Vec<i32> = vec![5, 1, 2, 3, 4];

        let actual = super::RotatedArray::from_vec(left_rotations, given);

        assert_eq!(expected, actual.into_iter().copied().collect::<Vec<_>>());
    }
}
