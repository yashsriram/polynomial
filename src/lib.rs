use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[macro_export]
macro_rules! polynomial (
    ($($power:expr => $coeff:expr),*) => (
        {
            let mut p = Polynomial::new();
            $(
                p.insert($power, $coeff);
            )*
            p
        }
    );
);

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    coeff_of_power: HashMap<i32, f32>,
}

impl Polynomial {
    pub fn new() -> Self {
        Polynomial {
            coeff_of_power: HashMap::new(),
        }
    }

    pub fn insert(&mut self, power: i32, coeff: f32) -> Option<f32> {
        if power < 0 {
            panic!("Power supplied to a polynomial term is < 0.");
        }
        self.coeff_of_power.insert(power, coeff)
    }

    pub fn at(&self, x: f32) -> f32 {
        let mut value = 0f32;
        for (&power, &coeff) in self.coeff_of_power.iter() {
            value += coeff * x.powi(power);
        }
        value
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sorted_coeff_of_power = {
            let mut map = self.coeff_of_power.iter().collect::<Vec<(&i32, &f32)>>();
            map.sort_by(|a, b| b.0.cmp(a.0));
            map
        };
        write!(f, "{:?}", sorted_coeff_of_power)
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sum = self.clone();
        for (&power, &coeff) in other.coeff_of_power.iter() {
            sum.insert(
                power,
                match sum.coeff_of_power.get(&power) {
                    Some(&prev_coeff) => prev_coeff + coeff,
                    None => coeff,
                },
            );
        }
        sum
    }
}

impl AddAssign for Polynomial {
    fn add_assign(&mut self, other: Self) {
        for (&power, &coeff) in other.coeff_of_power.iter() {
            self.insert(
                power,
                match self.coeff_of_power.get(&power) {
                    Some(&prev_coeff) => prev_coeff + coeff,
                    None => coeff,
                },
            );
        }
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut difference = self.clone();
        for (&power, &coeff) in other.coeff_of_power.iter() {
            difference.insert(
                power,
                match difference.coeff_of_power.get(&power) {
                    Some(&prev_coeff) => prev_coeff - coeff,
                    None => -coeff,
                },
            );
        }
        difference
    }
}

impl SubAssign for Polynomial {
    fn sub_assign(&mut self, other: Self) {
        for (&power, &coeff) in other.coeff_of_power.iter() {
            self.insert(
                power,
                match self.coeff_of_power.get(&power) {
                    Some(&prev_coeff) => prev_coeff - coeff,
                    None => -coeff,
                },
            );
        }
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut product = Polynomial::new();
        for (&a_power, &a_coeff) in self.coeff_of_power.iter() {
            let mut term_mul = Polynomial::new();
            for (&b_power, &b_coeff) in other.coeff_of_power.iter() {
                // FIXME
                term_mul.insert(a_power + b_power, a_coeff * b_coeff);
            }
            product += term_mul;
        }
        product
    }
}

#[cfg(test)]
mod tests {
    use super::{polynomial, Polynomial};

    #[test]
    fn at() {
        let p = polynomial! { 1 => 1.0, 2 => 5.0, 0 => 5.0, 3 => -2.0, 4 => -1.0, 5 => 1.0 };
        assert_eq!(p.at(3.0), 161.0);
    }

    #[test]
    fn add() {
        let p = polynomial! { 1 => 11.0, 2 => 57.0, 0 => 51.0 };
        let q = polynomial! { 3 => 73.0, 2 => -118.0, 0 => 40.0 };
        assert_eq!(
            p + q,
            polynomial! { 3 => 73.0, 2 => -61.0, 1 => 11.0, 0 => 91.0 }
        );
    }

    #[test]
    fn add_assign() {
        let mut p = polynomial! { 1 => 11.0, 2 => 57.0, 0 => 51.0 };
        let q = polynomial! { 3 => 73.0, 2 => -118.0, 0 => 40.0 };
        p += q;
        assert_eq!(
            p,
            polynomial! { 3 => 73.0, 2 => -61.0, 1 => 11.0, 0 => 91.0 }
        );
    }

    #[test]
    fn sub() {
        let p = polynomial! { 1 => 11.0, 2 => 57.0, 0 => 51.0 };
        let q = polynomial! { 3 => 73.0, 2 => -118.0, 0 => 40.0 };
        assert_eq!(
            p - q,
            polynomial! { 3 => -73.0, 2 => 175.0, 1 => 11.0, 0 => 11.0 }
        );
    }

    #[test]
    fn sub_assign() {
        let mut p = polynomial! { 1 => 11.0, 2 => 57.0, 0 => 51.0 };
        let q = polynomial! { 3 => 73.0, 2 => -118.0, 0 => 40.0 };
        p -= q;
        assert_eq!(
            p,
            polynomial! { 3 => -73.0, 2 => 175.0, 1 => 11.0, 0 => 11.0 }
        );
    }

    #[test]
    fn mul() {
        let p = polynomial! { 1 => 1.0, 2 => 5.0, 0 => 5.0 };
        let q = polynomial! { 3 => 7.0, 2 => -8.0, 0 => 4.0 };
        assert_eq!(
            p * q,
            polynomial! { 5 => 35.0, 4 => -33.0, 3 => 27.0, 2 => -20.0, 1 => 4.0, 0 => 20.0 }
        );
    }
}
