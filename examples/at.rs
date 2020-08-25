use polynomial::{polynomial, Polynomial};

fn main() {
    let p = polynomial! {2 => 1.0, 1 => -5.0, 0 => 6.0};
    println!("Value of '{}'", p);
    println!("\tat {:?} is {:?}", 0.0, p.at(0.0));
    println!("\tat {:?} is {:?}", 1.0, p.at(1.0));
    println!("\tat {:?} is {:?}", 1.5, p.at(1.5));
    println!("\tat {:?} is {:?}", 2.0, p.at(2.0));
    println!("\tat {:?} is {:?}", 2.5, p.at(2.5));
    println!("\tat {:?} is {:?}", 3.0, p.at(3.0));
    println!("\tat {:?} is {:?}", 3.5, p.at(2.5));
    println!("\tat {:?} is {:?}", 4.0, p.at(4.0));
}
