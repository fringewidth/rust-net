use rust_net::*;

fn main() {
    let mut mat: MatrixOwner<f32> = MatrixOwner::normal(4, 2, 0., 1.);
    let mut eye: MatrixOwner<f32> = MatrixOwner::eye(2);

    println!("Matrix A: \n{}", MatDisplay::to_string(&mat));
    
    println!("Matrix I: \n{}", MatDisplay::to_string(&eye));

    println!("AI: \n{}", MatDisplay::to_string(&matmul(&mat, &eye)));

    // println!("A: \n{}", MatDisplay::to_string(&mat));
    // println!("Aᵀ:\n{}", MatDisplay::to_string(&mat.t()));
    // let matT = mat.t_mut();
    // let mut matResult = matmul(&mat, &matT);
    // matResult = matmul(&matResult, &MatrixOwner::eye(matResult.rows()));

    // println!("{}", matResult.to_string());
}
