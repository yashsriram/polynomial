use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Sub};

#[macro_export]
macro_rules! polynomial (
    ($($key:expr => $value:expr),*) => (
        {
            let mut p = polynomial::Polynomial::new();
            $(
                p.insert($key, $value);
            )*
            p
        }
    );
);

#[derive(Debug, Clone)]
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
        self.coeff_of_power.insert(power, coeff)
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
