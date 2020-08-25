use polynomial::{polynomial, Polynomial};

fn main() {
    let p = polynomial! { 100 => 0.0, 50 => 0.0, 1 => 7.0 };
    let q = polynomial! { 5000 => 0.0, 1 => 7.0 };
    println!("p == q: {}", p == q);
}
