use std::ops::{Index, IndexMut};

pub mod ops;

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

impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        assert!(i < self.rows, "'i' out of bounds. Expected [0, {}) but got {}", self.rows, i);
        assert!(j < self.cols, "'j' out of bounds. Expected [0, {}) but got {}", self.cols, j);
        self.data.get((i * self.cols) + j).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        assert!(i < self.rows, "'i' out of bounds. Expected [0, {}) but got {}", self.rows, i);
        assert!(j < self.cols, "'j' out of bounds. Expected [0, {}) but got {}", self.cols, j);
        self.data.get_mut((i * self.cols) + j).unwrap()
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
mod index_test {
    use super::*;

    #[test]
    fn index_of_mat() {
        let m = mat![
            0, 1, 2;
            1, 2, 3;
        ];

        for (i, j) in m.iter() {
            let value = m[(i, j)];
            assert_eq!(value, i + j, "Invalid value. Expected {} but got {}.", i + j, value)
        }
    }

    #[test]
    #[should_panic(expected = "'i' out of bounds. Expected [0, 1) but got 1")]
    fn invalid_row_of_mat() {
        let m = mat![row 0, 1, 2];
        let _ = m[(1, 0)];
    }

    #[test]
    #[should_panic(expected = "'j' out of bounds. Expected [0, 1) but got 1")]
    fn invalid_col_of_mat() {
        let m = mat![col 0, 1, 2];
        let _ = m[(0, 1)];
    }
}
