use crate::{Matrix, MatrixView};

use super::data::MatrixOwner;
// use super::data::MatrixView;

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    vec,
};

impl<T: Display + Copy> Matrix<T> for MatrixOwner<T> {
    fn rows(&self) -> usize {
        self.rows
    }
    fn cols(&self) -> usize {
        self.cols
    }
    fn data(&self) -> &Vec<T> {
        return &self.data;
    }
}

impl<'a, T: Display + Copy> Matrix<T> for MatrixView<'a, T> {
    fn rows(&self) -> usize {
        self.rows
    }
    fn cols(&self) -> usize {
        self.cols
    }
    fn data(&self) -> &Vec<T> {
        return self.data;
    }
}

impl<T: Default + Copy> MatrixOwner<T> {
    pub fn null(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    pub fn t(&mut self) -> MatrixView<'_, T> {
        MatrixView {
            rows: self.cols,
            cols: self.rows,
            data: &mut self.data,
        }
    }
}

impl<T> Index<(usize, usize)> for MatrixOwner<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for MatrixOwner<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}

impl<'a, T> Index<(usize, usize)> for MatrixView<'a, T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<'a, T> IndexMut<(usize, usize)> for MatrixView<'a, T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}

impl<'a, T> MatrixView<'a, T> {
    pub fn from_owner(owner: &'a mut MatrixOwner<T>) -> Self {
        MatrixView {
            rows: owner.rows,
            cols: owner.cols,
            data: &mut owner.data,
        }
    }
}
