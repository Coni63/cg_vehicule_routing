use crate::city::City;

pub struct Solution {
    pub routes: Vec<usize>,
    pub score: u16,
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
}

impl Clone for Solution {
    fn clone(&self) -> Self {
        Solution {
            routes: self.routes.clone(),
            score: self.score,
        }
    }
}
