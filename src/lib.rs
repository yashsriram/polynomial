use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};

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

#[derive(Debug, Clone)]
pub struct Polynomial {
    coeff_of_power: HashMap<u32, f32>,
}

impl Polynomial {
    pub fn new() -> Self {
        Polynomial {
            coeff_of_power: HashMap::new(),
        }
    }

    pub fn insert(&mut self, power: u32, coeff: f32) -> Option<f32> {
        self.coeff_of_power.insert(power, coeff)
    }

    pub fn degree(&self) -> Option<u32> {
        self.coeff_of_power
            .iter()
            .filter(|(&_, &coeff)| coeff != 0.0)
            .map(|(&power, &_)| power)
            .max()
    }

    pub fn at(&self, x: f32) -> f32 {
        let mut value = 0f32;
        for (&power, &coeff) in self.coeff_of_power.iter() {
            value += coeff * x.powi(power as i32);
        }
        value
    }

    pub fn plot<'a>(
        polys: &[&Polynomial],
        l: f32,
        r: f32,
        num_samples: u32,
        filename: &str,
    ) -> Result<(), &'a str> {
        if num_samples < 2 {
            return Err("Requested less than 2 samples for plotting.");
        }
        use gnuplot::*;
        let mut fg = Figure::new();
        let axes = fg.axes2d();
        for poly in polys.iter() {
            axes.lines(
                (0..num_samples).map(|i| l + (r - l) * (i as f32 / (num_samples - 1) as f32)),
                (0..num_samples)
                    .map(|i| l + (r - l) * (i as f32 / (num_samples - 1) as f32))
                    .map(|x| poly.at(x)),
                &[Caption(&format!("{}", poly)), LineWidth(1.0)],
            );
        }
        axes.set_x_label("x", &[])
            .set_y_label("y", &[])
            .set_grid_options(true, &[LineStyle(SmallDot), Color("grey")])
            .set_x_grid(true)
            .set_y_grid(true)
            .set_title(
                &format!("plotted from {} to {} with {} samples", l, r, num_samples),
                &[],
            );
        fg.echo_to_file(&format!("{}.gnuplot", filename));
        Ok(())
    }

    pub fn derivative(&self) -> Self {
        let mut derivative_of_self = Self::new();
        for (&power, &coeff) in self.coeff_of_power.iter() {
            if power > 0 {
                derivative_of_self.insert(power - 1, power as f32 * coeff);
            }
        }
        derivative_of_self
    }

    pub fn integral(&self, c: f32) -> Self {
        let mut derivative_of_self = Self::new();
        for (&power, &coeff) in self.coeff_of_power.iter() {
            derivative_of_self.insert(power + 1, coeff / (power + 1) as f32);
        }
        derivative_of_self.insert(0, c);
        derivative_of_self
    }

    fn forward_eq_ignoring_zero_coeff_powers(&self, b: &Self) -> bool {
        for (&a_power, &a_coeff) in self.coeff_of_power.iter() {
            if a_coeff == 0.0 {
                continue;
            }
            let coeff_match = match b.coeff_of_power.get(&a_power) {
                Some(&b_coeff) => a_coeff == b_coeff,
                None => false,
            };
            if !coeff_match {
                return false;
            }
        }
        true
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        self.forward_eq_ignoring_zero_coeff_powers(other)
            && other.forward_eq_ignoring_zero_coeff_powers(self)
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sorted_coeff_of_power = {
            let mut map = self.coeff_of_power.iter().collect::<Vec<(&u32, &f32)>>();
            map.sort_by(|a, b| b.0.cmp(a.0));
            map
        };
        for (&power, &coeff) in sorted_coeff_of_power {
            if coeff < 0.0 {
                write!(f, "{}x^{{{}}}", coeff, power)?;
            } else {
                write!(f, "+{}x^{{{}}}", coeff, power)?;
            }
        }
        write!(f, "")
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sum = self;
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
        let mut difference = self;
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
            // Since any term * b will result in non-overlapping terms, simple insert can be used instead of repeated polynomial addition
            for (&b_power, &b_coeff) in other.coeff_of_power.iter() {
                term_mul.insert(a_power + b_power, a_coeff * b_coeff);
            }
            // Here there can be overlaps and hence polynomial addition is required
            product += term_mul;
        }
        product
    }
}

impl Div for Polynomial {
    type Output = Self;

    fn div(self, divisor: Self) -> Self {
        let divisor_degree = divisor
            .degree()
            .expect("Requested division with zero polynomial.");
        let dividend_degree_opt = self.degree();
        if let None = dividend_degree_opt {
            return Polynomial::new();
        }
        let dividend_degree = dividend_degree_opt.unwrap();
        if dividend_degree < divisor_degree {
            return Polynomial::new();
        }
        let dividend_degree_coeff = self.coeff_of_power.get(&dividend_degree).unwrap();
        let divisor_degree_coeff = divisor.coeff_of_power.get(&divisor_degree).unwrap();
        let multiplier = polynomial! { dividend_degree - divisor_degree => dividend_degree_coeff / divisor_degree_coeff };
        let quotient = multiplier;
        let remaining_dividend = {
            let mut remaining_dividend = self - quotient.clone() * divisor.clone();
            remaining_dividend.coeff_of_power.remove(&dividend_degree);
            remaining_dividend
        };
        quotient + remaining_dividend / divisor
    }
}

impl Rem for Polynomial {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        self.clone() - (self / other.clone()) * other
    }
}

#[cfg(test)]
mod tests {
    use super::{polynomial, Polynomial};

    #[test]
    fn degree() {
        assert_eq!(polynomial! { 100 => 1.0, 0 => 5.0 }.degree(), Some(100));
        assert_eq!(
            polynomial! { 1 => 1.0, 2 => 5.0, 0 => 5.0, 3 => -2.0, 4 => -1.0, 5 => 1.0 }.degree(),
            Some(5)
        );
        assert_eq!(
            polynomial! { 3 => -1.0, 2 => -10.0, 1 => 10.0, 0 => 15.0 }.degree(),
            Some(3)
        );
        assert_eq!(polynomial! { 1 => 10.0, 0 => 15.0 }.degree(), Some(1));
        assert_eq!(polynomial! { 0 => 15.0 }.degree(), Some(0));
        assert_eq!(Polynomial::new().degree(), None);
    }

    #[test]
    fn at() {
        let p = polynomial! { 1 => 1.0, 2 => 5.0, 0 => 5.0, 3 => -2.0, 4 => -1.0, 5 => 1.0 };
        assert_eq!(p.at(3.0), 161.0);
    }

    #[test]
    fn plot() {
        let p = polynomial! { 3 => -1.0, 2 => -10.0, 1 => 10.0, 0 => 15.0 };
        let q = polynomial! { 2 => -5.0, 1 => -1.0, 0 => 30.0 };
        let r = polynomial! { 1 => -100.0, 0 => 30.0 };
        assert_eq!(
            Polynomial::plot(&[&p, &q, &r], -13.0, 5.0, 50, "plot_test"),
            Ok(())
        );
        assert_eq!(
            Polynomial::plot(&[&p, &q, &r], -13.0, 5.0, 1, "should_not_exist"),
            Err("Requested less than 2 samples for plotting.")
        );
    }

    #[test]
    #[should_panic]
    fn plot_in_non_exisiting_dir() {
        let p = polynomial! { 3 => -1.0, 2 => -10.0, 1 => 10.0, 0 => 15.0 };
        assert_eq!(
            Polynomial::plot(&[&p], -13.0, 5.0, 50, "foobar/plot_test"),
            Ok(())
        );
    }

    #[test]
    fn derivative() {
        assert_eq!(polynomial! { 0 => 15.0 }.derivative(), Polynomial::new());
        assert_eq!(
            polynomial! { 1 => 10.0, 0 => 15.0 }.derivative(),
            polynomial! { 0 => 10.0 }
        );
        assert_eq!(
            polynomial! { 3 => -1.0, 2 => -10.0, 1 => 10.0, 0 => 15.0 }.derivative(),
            polynomial! { 2 => -3.0, 1 => -20.0, 0 => 10.0 }
        );
    }

    #[test]
    fn integral() {
        assert_eq!(Polynomial::new().integral(-5.0), polynomial! { 0 => -5.0 });
        assert_eq!(
            polynomial! { 0 => 10.0 }.integral(15.0),
            polynomial! { 1 => 10.0, 0 => 15.0 },
        );
        assert_eq!(
            polynomial! { 2 => -3.0, 1 => -20.0, 0 => 10.0 }.integral(15.0),
            polynomial! { 3 => -1.0, 2 => -10.0, 1 => 10.0, 0 => 15.0 },
        );
    }

    #[test]
    fn ignore_zero_coeff_for_eq() {
        assert_eq!(
            polynomial! { 4 => 0.0, 3 => 0.0, 2 => 0.0, 1 => 0.0 },
            Polynomial::new(),
        );
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

    #[test]
    fn div() {
        let p = Polynomial::new();
        let q = polynomial! { 1 => 1.0, 0 => -2.0 };
        assert_eq!(p / q, Polynomial::new());
        let p = polynomial! { 2 => 1.0, 1 => -5.0, 0 => 6.0 };
        let q = polynomial! { 1 => 1.0, 0 => -2.0 };
        assert_eq!(p / q, polynomial! { 1 => 1.0, 0 => -3.0});
        let p = polynomial! { 3 => 2.0, 2 => -5.0, 1 => -1.0, 0 => 3.0 };
        let q = polynomial! { 1 => 1.0, 0 => 3.0 };
        assert_eq!(p / q, polynomial! { 2 => 2.0, 1 => -11.0, 0 => 32.0});
        let p = polynomial! { 4 => 6.0, 3 => 5.0, 1 => 4.0, 0 => -4.0 };
        let q = polynomial! { 2 => 2.0, 1 => 1.0, 0 => -1.0 };
        assert_eq!(p / q, polynomial! { 2 => 3.0, 1 => 1.0, 0 => 1.0});
    }

    #[test]
    #[should_panic]
    fn div_with_zero_polynomial1() {
        let p = Polynomial::new();
        let q = Polynomial::new();
        let _ = p / q;
    }

    #[test]
    #[should_panic]
    fn div_with_zero_polynomial2() {
        let p = polynomial! { 3 => 2.0, 2 => -5.0, 1 => -1.0, 0 => 3.0 };
        let q = Polynomial::new();
        let _ = p / q;
    }

    #[test]
    fn rem() {
        let p = Polynomial::new();
        let q = polynomial! { 1 => 1.0, 0 => -2.0 };
        assert_eq!(p % q, Polynomial::new());
        let p = polynomial! { 2 => 1.0, 1 => -5.0, 0 => 6.0 };
        let q = polynomial! { 1 => 1.0, 0 => -2.0 };
        assert_eq!(p % q, Polynomial::new());
        let p = polynomial! { 3 => 2.0, 2 => -5.0, 1 => -1.0, 0 => 3.0 };
        let q = polynomial! { 1 => 1.0, 0 => 3.0 };
        assert_eq!(p % q, polynomial! { 0 => -93.0});
        let p = polynomial! { 4 => 6.0, 3 => 5.0, 1 => 4.0, 0 => -4.0 };
        let q = polynomial! { 2 => 2.0, 1 => 1.0, 0 => -1.0 };
        assert_eq!(p % q, polynomial! { 1 => 4.0, 0 => -3.0 });
    }

    #[test]
    #[should_panic]
    fn rem_with_zero_polynomial1() {
        let p = Polynomial::new();
        let q = Polynomial::new();
        let _ = p % q;
    }

    #[test]
    #[should_panic]
    fn rem_with_zero_polynomial2() {
        let p = polynomial! { 3 => 2.0, 2 => -5.0, 1 => -1.0, 0 => 3.0 };
        let q = Polynomial::new();
        let _ = p % q;
    }
}
