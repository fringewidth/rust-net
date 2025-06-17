use super::data::MatrixOwner;
use num_traits::One;
use rand::Rng;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

impl<T: Default + Clone> MatrixOwner<T> {
    pub fn null(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
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

impl<T: From<f32> + Clone> MatrixOwner<T> {
    pub fn normal(rows: usize, cols: usize, mean: f32, var: f32) -> Self {
        let dist = Normal::new(mean, var.sqrt()).unwrap();
        let mut rng = thread_rng();
        let data = (0..rows * cols)
            .map(|_| T::from(dist.sample(&mut rng)))
            .collect();
        Self { rows, cols, data }
    }
}
