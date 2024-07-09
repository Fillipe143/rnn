use std::ops::{Add, AddAssign};

use crate::Mat;

impl<T> Add<Mat<T>> for Mat<T> 
where 
    T: Add<Output = T> + Clone
{
    type Output = Mat<T>;

    fn add(self, m: Mat<T>) -> Self::Output {
        assert_eq!(self.rows, m.rows, "The number of rows in both matrices must be equals. But {} != {}", self.rows, m.rows);
        assert_eq!(self.cols, m.cols, "The number of cols in both matrices must be equals. But {} != {}", self.cols, m.cols);

        let mut output_mat = Mat::empty(self.rows, self.cols);
        for i in 0..(self.rows*self.cols) {
            output_mat.data.push(self.data[i].clone() + m.data[i].clone());
        }

        output_mat
    }
}

impl<T> AddAssign<Mat<T>> for Mat<T>
where 
    T: AddAssign<T> + Clone
{
    fn add_assign(&mut self, m: Mat<T>) {
        assert_eq!(self.rows, m.rows, "The number of rows in both matrices must be equals. But {} != {}", self.rows, m.rows);
        assert_eq!(self.cols, m.cols, "The number of cols in both matrices must be equals. But {} != {}", self.cols, m.cols);

        for i in 0..(self.rows*self.cols) {
            self.data[i] += m.data[i].clone();
        }
    }
}

impl<T> Add<T> for Mat<T>
where
    T: Add<Output = T> + Clone 
{
    type Output = Mat<T>;

    fn add(self, scalar: T) -> Self::Output {
        let mut output_mat = Mat::empty(self.rows, self.cols);
        for i in 0..(self.rows*self.cols) {
            output_mat.data.push(self.data[i].clone() + scalar.clone());
        }

        output_mat
    }
}


impl<T> AddAssign<T> for Mat<T> 
where
    T: AddAssign<T> + Clone
{
    fn add_assign(&mut self, scalar: T) {
        for i in 0..(self.rows*self.cols) {
            self.data[i] += scalar.clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mat;

    #[test]
    fn add_mat() {
        let a = mat![
            0, 1, 2;
            3, 4, 5;
        ];

        let b = mat![
            5, 4, 3;
            2, 1, 0;
        ];

        let c = a + b;
        for (idx, (i, j)) in c.iter().enumerate() {
            assert_eq!(c.data[idx], 5, "Invalid add at {} {}", i, j);
        }
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed: The number of rows in both matrices must be equals. But 1 != 2\n  left: 1\n right: 2")]
    fn add_invalid_mat() {
        let a = mat![0; 1, 2];
        let b = mat![0; 2, 1];
        let _ = a + b;
    }

    #[test]
    fn add_assign_mat() {
        let mut a = mat![
            0, 1, 2;
            3, 4, 5;
        ];

        a += mat![
            5, 4, 3;
            2, 1, 0;
        ];


        for (idx, (i, j)) in a.iter().enumerate() {
            assert_eq!(a.data[idx], 5, "Invalid add at {} {}", i, j);
        }
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed: The number of cols in both matrices must be equals. But 2 != 1\n  left: 2\n right: 1")]
    fn add_assign_invalid_mat() {
        let mut a = mat![0; 1, 2];
        let b = mat![0; 1, 1];
        a += b;
    }

    #[test]
    fn add_scalar_mat() {
        let a = mat![
            -1, 0, 1;
            2, 3, 4;
        ];

        let b = a + 1;

        for (idx, (i, j)) in b.iter().enumerate() {
            assert_eq!(b.data[idx], idx as i32, "Invalid add at {} {}", i, j);
        }
    }

    #[test]
    fn add_assign_scalar_mat() {
        let mut a = mat![
            -1, 0, 1;
            2, 3, 4;
        ];

        a += 1;

        for (idx, (i, j)) in a.iter().enumerate() {
            assert_eq!(a.data[idx], idx as i32, "Invalid add at {} {}", i, j);
        }
    }
}
