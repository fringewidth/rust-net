use rust_net::{Matrix, MatrixOwner};

fn main() {
    let mut mat: MatrixOwner<f32> = MatrixOwner::null(3, 4);
    println!("A: \n{}", Matrix::to_string(&mat));
    println!("Aᵀ:\n{}", Matrix::to_string(&mat.t()));
}
