use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Polynomial {
    coeff_of_power: HashMap<i32, f32>,
}

impl Polynomial {}

impl Default for Polynomial {
    fn default() -> Self {
        let map = {
            let mut map = HashMap::new();
            map.insert(2, 1.0);
            map.insert(1, -5.0);
            map.insert(0, 6.0);
            map
        };
        Polynomial {
            coeff_of_power: map,
        }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let map = {
            let mut map = self.coeff_of_power.iter().collect::<Vec<(&i32, &f32)>>();
            map.sort_by(|a, b| b.0.cmp(a.0));
            map
        };
        for (power, coeff) in map.iter() {
            write!(f, "{}x^{} ", coeff, power)?;
        }
        write!(f, "")
    }
}
