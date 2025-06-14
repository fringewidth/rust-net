use std::fmt::Display;

pub trait Matrix<T: Display + Copy> {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn data(&self) -> &Vec<T>;
    fn at(&self, index: (usize, usize)) -> T {
        self.data()[index.0 * self.cols() + index.1]
    }

    fn shape(&self) -> (usize, usize) {
        return (self.rows(), self.cols());
    }

    fn to_string(&self) -> String {
        let mut ret = String::from("[[");
        for i in 0..self.rows() {
            if i != 0 {
                ret += " ["
            }
            for j in 0..self.cols() {
                ret += self.at((i, j)).to_string().as_str();
                if j != self.cols() - 1 {
                    ret += "  ";
                }
            }
            if i != self.rows() - 1 {
                ret += "]\n";
            }
        }
        ret += "]]";
        ret
    }
}

pub struct MatrixOwner<T> {
    pub(super) rows: usize,
    pub(super) cols: usize,
    pub(super) data: Vec<T>,
}

pub struct MatrixView<'a, T> {
    pub(super) rows: usize,
    pub(super) cols: usize,
    pub(super) data: &'a mut Vec<T>,
}
