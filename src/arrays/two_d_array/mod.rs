use crate::lib;

/// Solution for:
/// https://www.hackerrank.com/challenges/2d-array/problem

#[allow(dead_code)]
pub fn main() {
    let matrix: Matrix<i8> = Matrix(lib::read_matrix_from_std(6, 6));
    let hourglasses = matrix.hourglasses();
    println!("{:}", hourglasses.0.max());
}

#[derive(Debug, PartialEq)]
struct Matrix<T>(Vec<Vec<T>>);
#[derive(Debug, PartialEq)]
struct Hourglasses<T>(Matrix<T>);

impl Matrix<i8> {

    fn hourglass_at(&self, i: usize, j: usize) -> i8 {
        let matrix = &self.0;
        let (w, h) = (matrix[0].len(), matrix.len());
        if i < 1 || i > h - 2 || j < 1 || j > w - 2 {
            panic!(format!("Hourglass at {:}, {:} is undefined for matrix {:}x{:}!", i, j, w, h));
        }
        let mut sum = 0;
        for ii in i-1..=i+1 {
            for jj in j-1..=j+1 {
                if jj != j && ii == i {
                    continue;
                }
                sum += matrix[ii][jj];
            }
        }
        sum
    }

    fn hourglasses(&self) -> Hourglasses<i8> {
        let matrix = &self.0;
        let (w, h) = (matrix[0].len(), matrix.len());
        let mut hs = vec![vec![0; w - 2]; h - 2];

        for i in 1..h-1 {
            for j in 1..w-1 {
                hs[i-1][j-1] = self.hourglass_at(i, j);
            }
        }
        Hourglasses(Matrix(hs))
    }

    fn max(&self) -> i8 {
        self.0.iter().map(|r| *r.iter().max().unwrap()).max().unwrap()
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matrix_hourglass_simple() {
        let matrix: Matrix<i8> = Matrix(vec![
            vec![1, 1, 1],
            vec![0, 1, 0],
            vec![1, 1, 1],
        ]);
        let expected: Hourglasses<i8> = Hourglasses(Matrix(vec![
            vec![7],
        ]));
        assert_eq!(expected, matrix.hourglasses());
    }

    #[test]
    fn matrix_hourglass_sample1() {
        let matrix: Matrix<i8> = Matrix(vec![
            vec![-9, -9, -9,  1, 1, 1, ],
            vec![0, -9,  0,  4, 3, 2,],
            vec![-9, -9, -9,  1, 2, 3,],
            vec![0,  0,  8,  6, 6, 0,],
            vec![0,  0,  0, -2, 0, 0,],
            vec![0,  0,  1,  2, 4, 0,],
        ]);
        let expected: Hourglasses<i8> = Hourglasses(Matrix(vec![
            vec![-63, -34, -9, 12, ],
            vec![-10, 0, 28, 23, ],
            vec![-27, -11, -2, 10, ],
            vec![9, 17, 25, 18,],
        ]));
        assert_eq!(expected, matrix.hourglasses());
    }

    #[test]
    fn matrix_hourglass_sample2() {
        let matrix: Matrix<i8> = Matrix(vec![
            vec![1, 1, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0],
            vec![0, 0, 2, 4, 4, 0],
            vec![0, 0, 0, 2, 0, 0],
            vec![0, 0, 1, 2, 4, 0],
        ]);
        let expected: Hourglasses<i8> = Hourglasses(Matrix(vec![
            vec![ 7,  4,  2,  0],
            vec![ 4,  8, 10,  8],
            vec![ 3,  6,  7,  6],
            vec![ 2,  9, 19, 14],
        ]));
        assert_eq!(expected, matrix.hourglasses());
    }

}