use polynomial::{polynomial, Polynomial};

fn main() {
    let p = Polynomial::new();
    println!("Polynomial '{}' has degree {:?}", p, p.degree());
    let p = polynomial! {100 => 0.0, 2 => 0.0, 0 => 0.0};
    println!("Polynomial '{}' has degree {:?}", p, p.degree());
    let p = polynomial! {100 => 0.0, 2 => 1.0, 0 => -1.0};
    println!("Polynomial '{}' has degree {:?}", p, p.degree());
    let p = polynomial! {100 => 1.0, 2 => 1.0, 0 => -1.0};
    println!("Polynomial '{}' has degree {:?}", p, p.degree());
}
