pub struct Matrix<T> {
    pub(super) rows: usize,
    pub(super) cols: usize,
    pub(super) data: Vec<T>,
}
