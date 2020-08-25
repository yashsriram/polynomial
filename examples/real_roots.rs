use polynomial::{polynomial, Polynomial};

fn main() {
    let polynomials = [
        (polynomial! {2 => 1.0, 0 => 1.0 }, (-2.0, 2.0)),
        (polynomial! {1 => 1.0, 0 => -1.0 }, (-2.0, 2.0)),
        (polynomial! {2 => 1.0, 0 => -1.0 }, (-2.0, 2.0)),
        (polynomial! {3 => 1.0, 1 => -1.0 }, (-2.0, 2.0)),
        (
            polynomial! {4 => 1.0, 3 => -22.0, 2 => 152.0, 1 => -362.0, 0 => 231.0},
            (0.0, 12.0),
        ),
        (
            polynomial! {4 => 1.0, 3 => 6.0, 2 => -337.0, 1 => -366.0, 0 => 2016.0},
            (-22.0, 17.0),
        ),
    ];
    for (i, (poly, range)) in polynomials.iter().enumerate() {
        let filename = format!("real_roots_{}", i + 1);
        let dx = 0.001;
        Polynomial::plot(&[&poly], range.0, range.1, 50, &filename).unwrap();
        println!(
            "'{}' is plotted in {}.gnuplot, found real roots (with precision = {}): {:?}",
            poly,
            filename,
            dx,
            poly.real_roots(dx)
        );
    }
}
