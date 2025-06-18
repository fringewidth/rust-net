use rust_net::*;

fn main() {
    let mut a: MatrixOwner<f32> = MatrixOwner::normal(3, 2, 0., 1.);
    let mut b: MatrixOwner<f32> = MatrixOwner::normal(2, 4, 0., 1.);

    println!("Matrix A:\n{}", MatDisplay::to_string(&a));
    println!("Matrix B:\n{}", MatDisplay::to_string(&b));

    // AB
    let mut ab = matmul(&a, &b);
    println!("AB:\n{}", MatDisplay::to_string(&ab));

    // Bᵀ
    let mut b_t = b.t_mut();
    println!("Bᵀ:\n{}", MatDisplay::to_string(&b_t));

    // Aᵀ
    let mut a_t = a.t_mut();
    println!("Aᵀ:\n{}", MatDisplay::to_string(&a_t));

    // (AB)^T
    let mut ab_t = ab.t_mut();
    println!("(AB)^T:\n{}", MatDisplay::to_string(&ab_t));

    // BᵀAᵀ
    let mut bta_t = matmul(&b_t, &a_t);
    println!("BᵀAᵀ:\n{}", MatDisplay::to_string(&bta_t));
}
