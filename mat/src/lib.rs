use std::ops::{Add, AddAssign};

#[macro_export]
macro_rules! mat {
    ($value:expr;$rows:expr,$cols:expr) => {
        Mat {
            rows: $rows,
            cols: $cols,
            data: vec![$value; ($rows) * ($cols)]
        }
    };

    ($($($value:expr),+);+;) => {
        {
            let mut tmp_mat = Mat {
                rows: 0,
                cols: 0,
                data: Vec::new()
            };

            $(
                let mut cols = 0;
                tmp_mat.rows += 1;
                $(
                    cols += 1;
                    tmp_mat.data.push($value);
                )+

                if tmp_mat.cols == 0 {
                    tmp_mat.cols = cols;
                } else {
                    assert!(tmp_mat.cols == cols, "Number of element in each row must be equal.");
                }
            )+

            tmp_mat
        }
    };

    (row$($value:expr),+) => {
        {
            let mut tmp_mat = Mat {
                rows: 1,
                cols: 0,
                data: Vec::new()
            };

            $(
                tmp_mat.cols += 1;
                tmp_mat.data.push($value);
            )+

            tmp_mat
        }
    };

    (col$($value:expr),+) => {
        {
            let mut tmp_mat = Mat {
                rows: 0,
                cols: 1,
                data: Vec::new()
            };

            $(
                tmp_mat.rows += 1;
                tmp_mat.data.push($value);
            )+

            tmp_mat
        }
    };
}

#[derive(Debug)]
pub struct Mat<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>
}

pub struct MatIterator<'a, T> {
    mat: &'a Mat<T>,
    i: usize,
    j: usize
}

impl<T> Mat<T>  {
    pub fn empty(rows: usize, cols: usize) -> Mat<T> {
        Mat { rows, cols, data: Vec::<T>::new() }
    }

    pub fn iter<'a>(&'a self) -> MatIterator<'a, T> {
        MatIterator { mat: self, i: 0, j: 0 }
    }
}

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

impl<'a, T> Iterator for MatIterator<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (curr_i, curr_j) = (self.i, self.j);

        if self.j + 1 >= self.mat.cols {
            self.j = 0;
            self.i += 1;
        } else {
            self.j += 1;
        }

        if curr_i >= self.mat.rows { None }
        else { Some((curr_i, curr_j)) }
    }
}

#[cfg(test)]
mod macro_tests {
    use super::*;

    #[test]
    fn create_mat_macro() {
        let m = mat![0; 2, 3];
        assert_eq!(m.rows, 2, "Invalid number of rows. Expected {} but got {}.", 2, m.rows);
        assert_eq!(m.cols, 3, "Invalid number of cols. Expected {} but got {}.", 3, m.cols);
        assert_eq!(m.data.len(), m.rows * m.cols, "Invalid number of elements. Expected {} but got {}.", m.rows*m.cols, m.data.len());
    }

    #[test]
    fn initialize_mat_macro() {
        let m = mat![
            0, 1;
            2, 3;
            4, 5;
        ];

        assert_eq!(m.rows, 3, "Invalid number of rows. Expected {} but got {}.", 2, m.rows);
        assert_eq!(m.cols, 2, "Invalid number of cols. Expected {} but got {}.", 3, m.cols);
        assert_eq!(m.data.len(), m.rows * m.cols, "Invalid number of elements. Expected {} but got {}.", m.rows*m.cols, m.data.len());
        for i in 0..5 { assert_eq!(m.data[i], i, "Invalid element in position. Expected {} but got {}.", i, m.data[i]); }
    }

    #[test]
    #[should_panic(expected = "Number of element in each row must be equal.")]
    fn initialize_invalid_mat_macro() {
        let _ = mat![
            1;
            2, 3, 4;
        ];
    }

    #[test]
    fn initialize_row_mat_macro() {
        let m = mat![row 0, 1, 2];

        assert_eq!(m.rows, 1, "Invalid number of rows. Expected {} but got {}.", 2, m.rows);
        assert_eq!(m.cols, 3, "Invalid number of cols. Expected {} but got {}.", 3, m.cols);
        assert_eq!(m.data.len(), m.rows * m.cols, "Invalid number of elements. Expected {} but got {}.", m.rows*m.cols, m.data.len());
        for i in 0..3 { assert_eq!(m.data[i], i, "Invalid element in position. Expected {} but got {}.", i, m.data[i]); }
    }

    #[test]
    fn initialize_col_mat_macro() {
        let m = mat![col 0, 1, 2];

        assert_eq!(m.rows, 3, "Invalid number of rows. Expected {} but got {}.", 2, m.rows);
        assert_eq!(m.cols, 1, "Invalid number of cols. Expected {} but got {}.", 3, m.cols);
        assert_eq!(m.data.len(), m.rows * m.cols, "Invalid number of elements. Expected {} but got {}.", m.rows*m.cols, m.data.len());
        for i in 0..3 { assert_eq!(m.data[i], i, "Invalid element in position. Expected {} but got {}.", i, m.data[i]); }
    }
}

#[cfg(test)]
mod iter_test {
    use super::*;

    #[test]
    fn iter_for_mat() {
        let m = mat![
            0, 1, 2;
            3, 4, 5;
        ];

        let iterations_count = m.iter().count();
        assert_eq!(iterations_count, m.rows*m.cols, "Invalid numer of iterations. Expected {} but got {}.", m.rows*m.cols, iterations_count);

        let expected = vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)];
        for (idx, (i, j)) in m.iter().enumerate() {
            let (e_i, e_j) = expected[idx];
            assert_eq!(i, e_i, "Invalid 'i'. Expected ({}, {}) but got ({}, {}).", e_i, e_j, i, j);
            assert_eq!(j, e_j, "Invalid 'j'. Expected ({}, {}) but got ({}, {}).", e_i, e_j, i, j);
        }
    }
}

#[cfg(test)]
mod op_test {
    use super::*;

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
}
