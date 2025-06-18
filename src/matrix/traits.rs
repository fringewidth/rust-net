use std::fmt::Display;

pub trait Matrix<T: Copy> {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn data(&self) -> &Vec<T>;
    fn transposed(&self) -> bool;
    fn at(&self, index: (usize, usize)) -> T {
        if !self.transposed() {
            self.data()[index.0 * self.cols() + index.1]
        } else {
            self.data()[index.1 * self.rows() + index.0]
        }
    }

    fn shape(&self) -> (usize, usize) {
        return (self.rows(), self.cols());
    }
}

pub trait MatDisplay<T: Display + Copy>: Matrix<T> {
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
