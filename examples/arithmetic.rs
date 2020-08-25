use polynomial::{polynomial, Polynomial};

fn main() {
    let p = polynomial! {2 => 1.0, 0 => -1.0};
    let q = polynomial! {1 => -1.0, 0 => 7.0};
    Polynomial::plot(&[&p, &q, &(&p + &q)], -4.0, 4.0, 50, "add").unwrap();
    Polynomial::plot(&[&p, &q, &(&p - &q)], -4.0, 4.0, 50, "sub").unwrap();
    Polynomial::plot(&[&p, &q, &(&p * &q)], -4.0, 4.0, 50, "mul").unwrap();
    Polynomial::plot(&[&p, &q, &(&p / &q)], -4.0, 4.0, 50, "div").unwrap();
    Polynomial::plot(&[&p, &q, &(&p % &q)], -4.0, 4.0, 50, "rem").unwrap();

    let p_original = polynomial! {2 => 1.0, 0 => -1.0};
    let mut p_add_assign = p_original.clone();
    p_add_assign += &polynomial! { 1 => 3.0 };
    let mut p_sub_assign = p_original.clone();
    p_sub_assign -= &polynomial! { 1 => 3.0 };
    Polynomial::plot(
        &[&p_original, &p_add_assign, &p_sub_assign],
        -4.0,
        4.0,
        50,
        "add_sub_assign",
    )
    .unwrap();
}
