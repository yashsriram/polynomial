use polynomial::{polynomial, Polynomial};

fn main() {
    let p = polynomial! { 0 => 6.7};
    let q = polynomial! { 1 => 10.0, 0 => -10.0};
    let r = polynomial! { 2 => 1.0, 0 => 1.0};
    let s = polynomial! { 3 => 1.0, 2 => -5.0, 1 => 6.0 };
    Polynomial::plot(&[&p, &q, &r, &s], -0.25, 4.0, 50, "plot").unwrap();
}
