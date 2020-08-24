use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};
mod tests;

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

/// Invariant: Only terms with non-zero coefficients are stored in memory.
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    coeff_of_power: HashMap<usize, f32>,
}

impl Polynomial {
    pub fn new() -> Self {
        Polynomial {
            coeff_of_power: HashMap::new(),
        }
    }

    pub fn insert(&mut self, power: usize, coeff: f32) {
        if coeff == 0.0 {
            self.coeff_of_power.remove(&power);
            return;
        }
        self.coeff_of_power.insert(power, coeff);
    }

    pub fn degree(&self) -> Option<usize> {
        self.coeff_of_power.iter().map(|(&power, &_)| power).max()
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
        num_samples: usize,
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

    fn postive_real_roots_given_positive_degree(&self, dx: f32) -> Vec<f32> {
        let derivatives = {
            let degree = self
                .degree()
                .expect("Zero polynomial provided. Please provide postive degree polynomial.");
            assert!(
                degree > 0,
                "Zero degree polynomial provided. Please provide postive degree polynomial."
            );
            let mut derivatives = Vec::<Polynomial>::with_capacity(degree);
            derivatives.push(self.derivative());
            for i in 1..degree {
                derivatives.push(derivatives[i - 1].derivative());
            }
            derivatives
        };
        fn do_continue(original: &Polynomial, derivatives: &[Polynomial], x: f32) -> bool {
            let all_derivatives_positive = derivatives.iter().all(|der| der.at(x) > 0.0);
            if original.at(x) > 0.0 && all_derivatives_positive {
                // Always increasing
                return false;
            }
            let all_derivatives_negative = derivatives.iter().all(|der| der.at(x) < 0.0);
            if original.at(x) < 0.0 && all_derivatives_negative {
                // Always decreasing
                return false;
            }
            true
        }
        let mut roots = Vec::new();
        let mut x = dx;
        let mut prev_val;
        while do_continue(self, &derivatives, x) {
            prev_val = self.at(x);
            x += dx;
            if self.at(x) * prev_val <= 0.0 {
                roots.push(x);
            }
        }
        roots
    }

    fn reflect_about_y_axis(&self) -> Self {
        let mut reflection = self.clone();
        for (power, coeff) in reflection.coeff_of_power.iter_mut() {
            if power % 2 == 1 {
                *coeff = -*coeff;
            }
        }
        reflection
    }

    /// - Time complexity for general polynomial = O(L/dx); L = largest root abs value.
    /// - For zero polynomial an empty vec is returned.
    /// - No guarantee on how many times a multiple root is returned.
    pub fn real_roots(&self, dx: f32) -> Vec<f32> {
        assert!(dx > 0.0, "dx should be positive.");
        // Zero-term polynomial (zero polynomial)
        if self.coeff_of_power.len() == 0 {
            return vec![];
        }
        // One-term polynomial
        if self.coeff_of_power.len() == 1 {
            match self.coeff_of_power.iter().next() {
                Some((&power, &_)) => {
                    if power == 0 {
                        return vec![];
                    } else {
                        return vec![0.0];
                    }
                }
                None => (),
            }
        }
        // Multiple-term polynomial
        // Zero as a root
        let mut roots = Vec::new();
        if self.at(0.0) == 0.0 {
            roots.push(0.0);
        }
        // Positive roots
        let positive_roots = self.postive_real_roots_given_positive_degree(dx);
        roots.extend(positive_roots);
        // Negative roots
        let negative_roots = self
            .reflect_about_y_axis()
            .postive_real_roots_given_positive_degree(dx)
            .iter()
            .map(|root| -root)
            .collect::<Vec<f32>>();
        roots.extend(negative_roots);
        roots
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sorted_coeff_of_power = {
            let mut map = self.coeff_of_power.iter().collect::<Vec<(&usize, &f32)>>();
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
