#[derive(Clone)]
pub struct MatrixOwner<T> {
    pub(super) rows: usize,
    pub(super) cols: usize,
    pub(super) data: Vec<T>,
    pub(super) transposed: bool,
}
#[derive(Clone)]
pub struct MatrixView<'a, T> {
    pub(super) rows: usize,
    pub(super) cols: usize,
    pub(super) data: &'a Vec<T>,
    pub(super) transposed: bool,
}
