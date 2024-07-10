use std::ops::{AddAssign, Mul, MulAssign};

use crate::Mat;

impl<T> Mul<Mat<T>> for Mat<T> 
where 
    T: Mul<Output = T> + AddAssign<T> + Clone + Default
{
    type Output = Mat<T>;

    fn mul(self, m: Mat<T>) -> Self::Output {
        assert_eq!(self.cols, m.rows, "The number of columns must be equal to the number of rows in the second matrix. But {} != {}", self.cols, m.rows);

        let mut output_mat = Mat::empty(self.rows, m.cols);
        for i in 0..self.rows {
            for j in 0..m.cols {
                output_mat.data.push(T::default());
                for k in 0..self.cols {
                    output_mat[(i, j)] += self[(i, k)].clone() * m[(k, j)].clone()
                }
            }
        }

        output_mat
    }
}

impl<T> Mul<T> for Mat<T>
where
    T: Mul<Output = T> + Clone
{
    type Output = Mat<T>;

    fn mul(self, scalar: T) -> Self::Output {
        let mut output_mat = Mat::empty(self.rows, self.cols);
        for i in 0..(self.rows*self.cols) {
            output_mat.data.push(self.data[i].clone() * scalar.clone());
        }

        output_mat
    }
}

impl<T> MulAssign<T> for Mat<T>
where
    T: MulAssign<T> + Clone
{
    fn mul_assign(&mut self, scalar: T) {
        for i in 0..(self.rows*self.cols) {
            self.data[i] *= scalar.clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mat;

    #[test]
    fn mul_mat() {
        let a = mat![
            0, 1, 2;
            3, 4, 5;
        ];

        let b = mat![
            0, 1, 2, 3;
            4, 5, 6, 7;
            8, 9, 10, 11;
        ];

        let c = a * b;
        assert_eq!(c.rows, 2, "Invalid number of rows. Expected {} but got {}", 2, c.rows);
        assert_eq!(c.cols, 4, "Invalid number of cols. Expected {} but got {}", 4, c.cols);

        let output = mat![
            20, 23, 26, 29;
            56, 68, 80, 92;
        ];

        for (i, j) in c.iter() {
            assert_eq!(c[(i, j)], output[(i, j)], "Invalid mul at {} {}", i, j);
        }
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed: The number of columns must be equal to the number of rows in the second matrix. But 2 != 1")]
    fn mul_invalid_mat() {
        let a = mat![0; 1, 2];
        let b = mat![0; 1, 1];
        let _ = a * b;
    }

    #[test]
    fn mul_scalar_mat() {
        let a = mat![
            0, 1, 2;
            3, 4, 5;
        ];

        let b = a * 2;

        for (idx, (i, j)) in b.iter().enumerate() {
            let output = idx * 2;
            assert_eq!(b.data[idx], output, "Invalid mul at {} {}", i, j);
        }
    }

    #[test]
    fn mul_assign_scalar_mat() {
        let mut a = mat![
            0, 1, 2;
            3, 4, 5;
        ];

        a *= 2;

        for (idx, (i, j)) in a.iter().enumerate() {
            let output = idx * 2;
            assert_eq!(a.data[idx], output, "Invalid mul at {} {}", i, j);
        }
    }
}
