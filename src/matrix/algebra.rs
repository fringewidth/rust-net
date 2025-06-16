use super::{Matrix, MatrixOwner};
use std::ops::{Add, AddAssign, Mul};
use num_traits::{One, Signed};

pub fn matmul<T, A, B>(matA: &A, matB: &B) -> MatrixOwner<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Default + One,
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

pub fn scale<T, A>(matA: &A, scale: T) -> MatrixOwner<T>
where
    T: Copy + Default + Mul<Output = T>,
    A: Matrix<T>,
{
    let mut matB: MatrixOwner<T> = MatrixOwner::null(matA.rows(), matA.cols());
    for i in 0..matA.rows() {
        for j in 0..matA.cols() {
            matB[(i, j)] = matA.at((i, j)) * scale;
        }
    }
    matB
}

pub fn matadd<T, A, B>(matA: &A, matB: &B) -> MatrixOwner<T>
where
    T: Copy + Add<Output = T> + Default + AddAssign,
    A: Matrix<T>,
    B: Matrix<T>,
{
    assert_eq!(matA.rows(), matB.rows());
    assert_eq!(matA.cols(), matB.cols());
    let mut matC: MatrixOwner<T> = MatrixOwner::null(matA.rows(), matA.cols());
    for i in 0..matA.rows() {
        for j in 0..matA.cols() {
            matC[(i, j)] += matA.at((i, j)) + matB.at((i, j));
        }
    }
    matC
}

pub fn matsub<T, A, B>(matA: &A, matB: &B) -> MatrixOwner<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Default + AddAssign + One + Signed,
    A: Matrix<T>,
    B: Matrix<T>,
{
    assert_eq!(matA.rows(), matB.rows());
    assert_eq!(matA.cols(), matB.cols());
    let subtrahend = scale(matB, -T::one());
    matadd(matA, &subtrahend)
}
