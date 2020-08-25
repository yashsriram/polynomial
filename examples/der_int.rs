use polynomial::{polynomial, Polynomial};

fn main() {
    let p = polynomial! { 3 => 1.0, 2 => -5.0, 1 => 6.0, 0 => 11.0 };
    println!("Derivative of '{}' is '{}'", p, p.derivative());
    let q = polynomial! { 2 => 3.0, 1 => -10.0, 0 => 6.0 };
    println!(
        "Integral of '{}' with c = 11.0 is '{}'",
        q,
        q.integral(11.0)
    );
}
