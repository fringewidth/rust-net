use rust_net::*;

fn main() {
    let mut mat: MatrixOwner<f32> = MatrixOwner::null(3, 4);
    println!("A: \n{}", MatDisplay::to_string(&mat));
    println!("Aᵀ:\n{}", MatDisplay::to_string(&mat.t()));
    let matT = mat.t_mut();
    let mut matResult = matmul(&mat, &matT);
    matResult = matmul(&matResult, &MatrixOwner::eye(matResult.rows()));

    println!("{}", matResult.to_string());
}
