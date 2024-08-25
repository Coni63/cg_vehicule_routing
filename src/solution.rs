use std::collections::hash_map::DefaultHasher;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};

use crate::city::City;

pub struct Solution {
    pub routes: Vec<usize>,
    pub score: u32,
    pub hashcode: u64,
}

impl Solution {
    pub fn to_string(&self, cities: &[City], capacity: u16) -> String {
        let mut s = String::new();

        let mut remaining_capacity = capacity;
        for city_id in self.routes.iter() {
            if remaining_capacity >= cities[*city_id].get_demand() {
                s.push(' ');
            } else {
                s.push(';');
                remaining_capacity = capacity;
            }

            remaining_capacity -= cities[*city_id].get_demand();
            s.push_str(&city_id.to_string());
        }

        s
    }

    pub fn calculate_hash(&mut self, hasher: &mut DefaultHasher) {
        self.routes.hash(hasher);
        self.hashcode = hasher.finish()
    }
}

impl Clone for Solution {
    fn clone(&self) -> Self {
        Solution {
            routes: self.routes.clone(),
            score: self.score,
            hashcode: self.hashcode,
        }
    }
}

impl Debug for Solution {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Solution {{ routes: [")?;
        for (i, route) in self.routes.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", route)?;
        }
        write!(f, "], score: {} }}", self.score)?;
        Ok(())
    }
}
