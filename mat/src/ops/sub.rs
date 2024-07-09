use std::ops::{Sub, SubAssign};

use crate::Mat;

impl<T> Sub<Mat<T>> for Mat<T> 
where 
    T: Sub<Output = T> + Clone
{
    type Output = Mat<T>;

    fn sub(self, m: Mat<T>) -> Self::Output {
        assert_eq!(self.rows, m.rows, "The number of rows in both matrices must be equals. But {} != {}", self.rows, m.rows);
        assert_eq!(self.cols, m.cols, "The number of cols in both matrices must be equals. But {} != {}", self.cols, m.cols);

        let mut output_mat = Mat::empty(self.rows, self.cols);
        for i in 0..(self.rows*self.cols) {
            output_mat.data.push(self.data[i].clone() - m.data[i].clone());
        }

        output_mat
    }
}

impl<T> SubAssign<Mat<T>> for Mat<T>
where 
    T: SubAssign<T> + Clone
{
    fn sub_assign(&mut self, m: Mat<T>) {
        assert_eq!(self.rows, m.rows, "The number of rows in both matrices must be equals. But {} != {}", self.rows, m.rows);
        assert_eq!(self.cols, m.cols, "The number of cols in both matrices must be equals. But {} != {}", self.cols, m.cols);

        for i in 0..(self.rows*self.cols) {
            self.data[i] -= m.data[i].clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mat;

    #[test]
    fn sub_mat() {
        let a = mat![
            1, 2, 3;
            4, 5, 6;
        ];

        let b = mat![
            0, 1, 2;
            3, 4, 5;
        ];

        let c = a - b;
        for (idx, (i, j)) in c.iter().enumerate() {
            assert_eq!(c.data[idx], 1, "Invalid sub at {} {}", i, j);
        }
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed: The number of rows in both matrices must be equals. But 1 != 2\n  left: 1\n right: 2")]
    fn sub_invalid_mat() {
        let a = mat![0; 1, 2];
        let b = mat![0; 2, 1];
        let _ = a - b;
    }

    #[test]
    fn sub_assign_mat() {
        let mut a = mat![
            1, 2, 3;
            4, 5, 6;
        ];

        a -= mat![
            0, 1, 2;
            3, 4, 5;
        ];

        for (idx, (i, j)) in a.iter().enumerate() {
            assert_eq!(a.data[idx], 1, "Invalid sub at {} {}", i, j);
        }
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed: The number of cols in both matrices must be equals. But 2 != 1\n  left: 2\n right: 1")]
    fn sub_assign_invalid_mat() {
        let mut a = mat![0; 1, 2];
        let b = mat![0; 1, 1];
        a -= b;
    }
}
