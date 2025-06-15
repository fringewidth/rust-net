use crate::MatDisplay;

use super::data::{MatrixOwner, MatrixView};
use super::traits::Matrix;
use num_traits::One;

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    vec,
};

impl<T: Copy> Matrix<T> for MatrixOwner<T> {
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

impl<'a, T: Copy> Matrix<T> for MatrixView<'a, T> {
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
            data: &self.data,
        }
    }

    pub fn t_mut(&mut self) -> MatrixOwner<T> {
        MatrixOwner {
            rows: self.cols,
            cols: self.rows,
            data: self.data.clone(),
        }
    }
}

impl<T: Default + Copy + One> MatrixOwner<T> {
    pub fn eye(dim: usize) -> Self {
        let mut eye = Self {
            rows: dim,
            cols: dim,
            data: vec![T::default(); dim * dim],
        };
        for i in 0..dim {
            eye[(i, i)] = T::one();
        }
        eye
    }
}

impl<T: Copy> Index<(usize, usize)> for MatrixOwner<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<T: Copy> IndexMut<(usize, usize)> for MatrixOwner<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}

impl<'a, T: Copy> Index<(usize, usize)> for MatrixView<'a, T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

// impl<'a, T: Copy> IndexMut<(usize, usize)> for MatrixView<'a, T> {
//     fn index_mut(&mut self, (row, col): (usize, usize)) -> &Self::Output {
//         return &self.data[row * self.cols + col];
//     }
// }

impl<'a, T> MatrixView<'a, T> {
    pub fn from_owner(owner: &'a mut MatrixOwner<T>) -> Self {
        MatrixView {
            rows: owner.rows,
            cols: owner.cols,
            data: &mut owner.data,
        }
    }
}

impl<T: Copy + Display> MatDisplay<T> for MatrixOwner<T> {}
impl<'a, T: Copy + Display> MatDisplay<T> for MatrixView<'a, T> {}
