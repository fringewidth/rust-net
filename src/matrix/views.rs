use crate::MatDisplay;

use super::data::{MatrixOwner, MatrixView};
use super::traits::Matrix;

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
        &self.data
    }
    fn transposed(&self) -> bool {
        self.transposed
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
    fn transposed(&self) -> bool {
        self.transposed
    }
}

impl<T: Copy> MatrixOwner<T> {
    pub fn t(&mut self) -> MatrixView<'_, T> {
        MatrixView {
            rows: self.cols,
            cols: self.rows,
            data: &self.data,
            transposed: true,
        }
    }

    pub fn t_mut(&mut self) -> MatrixOwner<T> {
        MatrixOwner {
            rows: self.cols,
            cols: self.rows,
            data: self.data.clone(),
            transposed: true,
        }
    }
}

impl<T: Copy> Index<(usize, usize)> for MatrixOwner<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let idx = if self.transposed {
            col * self.rows + row
        } else {
            row * self.cols + col
        };
        &self.data[idx]
    }
}

impl<T: Copy> IndexMut<(usize, usize)> for MatrixOwner<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let idx = if self.transposed {
            col * self.rows + row
        } else {
            row * self.cols + col
        };
        &mut self.data[idx]
    }
}

impl<'a, T: Copy> Index<(usize, usize)> for MatrixView<'a, T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let idx = if self.transposed {
            col * self.rows + row
        } else {
            row * self.cols + col
        };
        &self.data[idx]
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
            transposed: false,
        }
    }
}

impl<T: Copy + Display> MatDisplay<T> for MatrixOwner<T> {}
impl<'a, T: Copy + Display> MatDisplay<T> for MatrixView<'a, T> {}
