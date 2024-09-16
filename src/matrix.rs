use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: u8,
    cols: u8,
    data: Vec<f32>,
}

impl Matrix {

    pub fn new(rows: u8, cols: u8, data: Vec<f32>) -> Self {
        assert_eq!(rows as usize * cols as usize, data.len(), "wrong dimensions");
        Self { rows, cols, data }
    }

    pub fn transpose(&self) -> Self {
        let mut transposed_data = vec![0.0; self.data.len()];
        for r in 0..self.rows {
            for c in 0..self.cols {
                transposed_data[c as usize * self.rows as usize + r as usize] =
                    self.data[r as usize * self.cols as usize + c as usize];
            }
        }
        Matrix::new(self.cols, self.rows, transposed_data)
    }

    pub fn negate(&self) -> Self {
        let neg_data: Vec<f32> = self.data.iter().map(|&x| -x).collect();
        Matrix::new(self.rows, self.cols, neg_data)
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows, "wrong dimensions");
        let mut result_data = vec![0.0; (self.rows * other.cols) as usize];

        for r in 0..self.rows {
            for c in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[r as usize * self.cols as usize + k as usize]
                        * other.data[k as usize * other.cols as usize + c as usize];
                }
                result_data[r as usize * other.cols as usize + c as usize] = sum;
            }
        }

        Matrix::new(self.rows, other.cols, result_data)
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows, "wrong dimensions");
        assert_eq!(self.cols, other.cols, "wrong dimensions");

        let sum_data: Vec<f32> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&x, &y)| x + y)
            .collect();

        Matrix::new(self.rows, self.cols, sum_data)
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows, "wrong dimensions");
        assert_eq!(self.cols, other.cols, "wrong dimensions");

        let diff_data: Vec<f32> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&x, &y)| x - y)
            .collect();

        Matrix::new(self.rows, self.cols, diff_data)
    }
}

