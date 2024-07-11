use crate::Mat;

type MatTransformer<I, O> = fn(value: I) -> O;

impl<T> Mat<T> 
where
    T: Clone
{
    pub fn transform<O>(&self, transformer: MatTransformer<T, O>) -> Mat<O> {
        let mut output: Mat<O> = Mat::empty(self.rows, self.cols);

        for i in 0..(self.rows*self.cols) {
            output.data.push(transformer(self.data[i].clone()));
        }

        output
    }

    pub fn transform_assign(&mut self, transformer: MatTransformer<T, T>) {
        for i in 0..(self.rows*self.cols) {
            self.data[i] = transformer(self.data[i].clone())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mat;

    #[test]
    fn transform_mat() {
        let m = mat![
            0, 1, 2;
            3, 4, 5;
        ];

        let half_m = m.transform(|v| (v as f32) / 2.0);
        for (idx, (i, j)) in half_m.iter().enumerate() {
            assert_eq!(half_m[(i, j)], (idx as f32) / 2.0, "Invalid transformation at {}, {}", i, j);
        }
    }

    #[test]
    fn transform_assign_mat() {
        let mut m = mat![
            0, 1, 2;
            3, 4, 5;
        ];

        m.transform_assign(|v| v * 2);
        for (idx, (i, j)) in m.iter().enumerate() {
            assert_eq!(m[(i, j)], idx * 2, "Invalid transformation at {}, {}", i, j);
        }
    }
}
