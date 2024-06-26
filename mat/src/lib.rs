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
                let mut rows = 0;
                tmp_mat.cols += 1;
                $(
                    rows += 1;
                    tmp_mat.data.push($value);
                )+

                if tmp_mat.rows == 0 {
                    tmp_mat.rows = rows;
                } else {
                    assert!(tmp_mat.rows == rows, "Number of element in each row must be equal.");
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

#[cfg(test)]
mod tests {
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

        assert_eq!(m.rows, 2, "Invalid number of rows. Expected {} but got {}.", 2, m.rows);
        assert_eq!(m.cols, 3, "Invalid number of cols. Expected {} but got {}.", 3, m.cols);
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