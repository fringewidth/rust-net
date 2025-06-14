use super::data::Matrix;

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    vec,
};

impl<T: Default + Clone> Matrix<T> {
    pub fn null(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }
}

impl<T: Display> Matrix<T> {
    pub fn to_string(&self) -> String {
        let mut ret = String::from("[[");
        for i in 0..self.rows {
            if i != 0 {
                ret += " ["
            }
            for j in 0..self.cols {
                ret += self[(i, j)].to_string().as_str();
                if j != self.cols - 1 {
                    ret += "  ";
                }
            }
            if i != self.rows - 1 {
                ret += "]\n";
            }
        }
        ret += "]]";
        ret
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}
