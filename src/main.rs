use polynomial::polynomial;

fn main() {
    let p = polynomial! {
        1 => 1.0,
        2 => 5.0
    };
    println!("{}", p);
    let q = polynomial! {
        2 => -12.0,
        3 => 10.0
    };
    println!("{}", q);
    println!("{}", p + q);
    let p = polynomial! {
        1 => 1.0,
        2 => 5.0
    };
    println!("{}", p);
    let q = polynomial! {
        2 => -12.0,
        3 => 10.0
    };
    println!("{}", q);
    println!("{}", p - q);
}
