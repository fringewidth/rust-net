use super::{Matrix, MatrixOwner};
use std::{
    ops::{Add, Mul},
    process::Output,
};

pub fn matmul<T, A, B>(matA: &A, matB: &B) -> MatrixOwner<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Default,
    for<'a, 'b> &'a T: Mul<&'b T, Output = T>,
    A: Matrix<T>,
    B: Matrix<T>,
{
    assert_eq!(
        matA.cols(),
        matB.rows(),
        "Matrices of incompatible types: {:?}, {:?}",
        matA.shape(),
        matB.shape()
    );

    let mut matC: MatrixOwner<T> = MatrixOwner::null(matA.rows(), matB.cols());

    for i in 0..matA.rows() {
        for j in 0..matB.cols() {
            for k in 0..matA.cols() {
                matC[(i, j)] = matA.at((i, k)) * matB.at((k, j));
            }
        }
    }

    matC
}
