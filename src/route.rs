use crate::{city::City, distance::Distance};
use std::fmt;

pub struct Route<'a> {
    cities: Vec<usize>,
    total_distance: u16,
    used_capacity: u16,
    distances: &'a Distance,
}

impl<'a> Route<'a> {
    pub fn new(distances: &'a Distance) -> Route {
        Route {
            cities: Vec::new(),
            total_distance: 0,
            used_capacity: 0,
            distances,
        }
    }

    pub fn get(&self, idx: usize) -> usize {
        self.cities[idx]
    }

    pub fn can_accept(&self, city: &City, max_capacity: u16) -> bool {
        self.used_capacity + city.get_demand() <= max_capacity
    }

    pub fn can_swap(&self, previous_city: &City, new_city: &City, max_capacity: u16) -> bool {
        self.used_capacity - previous_city.get_demand() + new_city.get_demand() <= max_capacity
    }

    pub fn add_city(&mut self, city: &City, position: usize) {
        let new_idx = city.get_index();
        let before_idx: usize;
        let after_idx: usize;

        if self.cities.is_empty() {
            before_idx = 0;
            after_idx = 0;
            self.cities.push(new_idx);
        } else if position >= self.cities.len() {
            before_idx = *self.cities.last().unwrap();
            after_idx = 0;
            self.cities.push(new_idx);
        } else {
            before_idx = if position == 0 {
                0
            } else {
                *self.cities.get(position - 1).unwrap_or(&0)
            };
            after_idx = *self.cities.get(position).unwrap();
            self.cities.insert(position, new_idx);
        }
        self.used_capacity += city.get_demand();
        self.total_distance = self.total_distance - self.distances.get(before_idx, after_idx)
            + self.distances.get(before_idx, new_idx)
            + self.distances.get(new_idx, after_idx);
    }

    pub fn remove_city(&mut self, city: &City) {
        let removed_idx = city.get_index();
        if let Some(position) = self.cities.iter().position(|x| *x == removed_idx) {
            let before_idx = if position == 0 {
                0
            } else {
                *self.cities.get(position - 1).unwrap_or(&0)
            };
            let after_idx = *self.cities.get(position + 1).unwrap_or(&0);
            self.used_capacity -= city.get_demand();
            self.total_distance = self.total_distance + self.distances.get(before_idx, after_idx)
                - self.distances.get(before_idx, removed_idx)
                - self.distances.get(removed_idx, after_idx);
            self.cities.remove(position);
        }
    }

    pub fn swap_city(&mut self, city_to_replace: &City, new_city: &City) {
        let removed_idx = city_to_replace.get_index();
        let new_idx = new_city.get_index();
        if let Some(position) = self.cities.iter().position(|x| *x == removed_idx) {
            let before_idx = if position == 0 {
                0
            } else {
                *self.cities.get(position - 1).unwrap_or(&0)
            };
            let after_idx = *self.cities.get(position + 1).unwrap_or(&0);
            self.cities[position] = new_idx;
            self.used_capacity =
                self.used_capacity - city_to_replace.get_demand() + new_city.get_demand();
            self.total_distance = self.total_distance
                + self.distances.get(before_idx, new_idx)
                + self.distances.get(new_idx, after_idx)
                - self.distances.get(before_idx, removed_idx)
                - self.distances.get(removed_idx, after_idx);
        }
    }

    pub fn inner_swap_city(&mut self, position_city_a: usize, position_city_b: usize) {
        if position_city_a == position_city_b {
            return;
        }

        let idx_city_a = self.cities[position_city_a];
        let idx_city_b = self.cities[position_city_b];

        let before_idx_a = if position_city_a == 0 {
            0
        } else {
            *self.cities.get(position_city_a - 1).unwrap_or(&0)
        };
        let after_idx_a = *self.cities.get(position_city_a + 1).unwrap_or(&0);

        let before_idx_b = if position_city_b == 0 {
            0
        } else {
            *self.cities.get(position_city_b - 1).unwrap_or(&0)
        };
        let after_idx_b = *self.cities.get(position_city_b + 1).unwrap_or(&0);

        self.total_distance = self.total_distance
            - self.distances.get(before_idx_a, idx_city_a)
            - self.distances.get(idx_city_a, after_idx_a)
            - self.distances.get(before_idx_b, idx_city_b)
            - self.distances.get(idx_city_b, after_idx_b);

        self.cities[position_city_a] = idx_city_b;
        self.cities[position_city_b] = idx_city_a;

        // splitted becquse there is error in cqse of swap of 2 consecutives values
        // TODO: optimize

        let before_idx_a = if position_city_a == 0 {
            0
        } else {
            *self.cities.get(position_city_a - 1).unwrap_or(&0)
        };
        let after_idx_a = *self.cities.get(position_city_a + 1).unwrap_or(&0);

        let before_idx_b = if position_city_b == 0 {
            0
        } else {
            *self.cities.get(position_city_b - 1).unwrap_or(&0)
        };
        let after_idx_b = *self.cities.get(position_city_b + 1).unwrap_or(&0);

        self.total_distance = self.total_distance
            + self.distances.get(before_idx_a, idx_city_b)
            + self.distances.get(idx_city_b, after_idx_a)
            + self.distances.get(before_idx_b, idx_city_a)
            + self.distances.get(idx_city_a, after_idx_b);
    }

    pub fn get_capacity(&self) -> u16 {
        self.used_capacity
    }

    pub fn get_total_distance(&self) -> u16 {
        self.total_distance
    }

    pub fn len(&self) -> usize {
        self.cities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cities.is_empty()
    }
}

impl<'a> Clone for Route<'a> {
    fn clone(&self) -> Route<'a> {
        Route {
            cities: self.cities.clone(),
            total_distance: self.total_distance,
            used_capacity: self.used_capacity,
            distances: self.distances,
        }
    }
}

impl<'a> fmt::Debug for Route<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Route:\nCapacity: {}\nDistance: {}\nCities: {:?}",
            self.used_capacity, self.total_distance, self.cities
        )?;
        Ok(())
    }
}

impl<'a> ToString for Route<'a> {
    fn to_string(&self) -> String {
        self.cities
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_city() {
        let cities = vec![
            City::new(0, 0, 0, 0),
            City::new(1, 10, 0, 2),
            City::new(2, 10, 10, 3),
            City::new(3, 0, 10, 4),
            City::new(4, 5, 5, 6),
        ];

        let distances = Distance::new(&cities);

        let mut route = Route::new(&distances);

        route.add_city(&cities[1], 0);
        assert_eq!(route.cities, vec![1]);
        assert_eq!(route.total_distance, 20);
        assert_eq!(route.used_capacity, 2);

        route.add_city(&cities[2], 1);
        route.add_city(&cities[3], 2);
        assert_eq!(route.cities, vec![1, 2, 3]);
        assert_eq!(route.total_distance, 40);
        assert_eq!(route.used_capacity, 9);

        route.add_city(&cities[4], 1);
        assert_eq!(route.cities, vec![1, 4, 2, 3]);
        assert_eq!(route.total_distance, 44);
        assert_eq!(route.used_capacity, 15);

        route.remove_city(&cities[2]);
        assert_eq!(route.cities, vec![1, 4, 3]);
        assert_eq!(route.total_distance, 34);
        assert_eq!(route.used_capacity, 12);

        route.swap_city(&cities[4], &cities[2]);
        assert_eq!(route.cities, vec![1, 2, 3]);
        assert_eq!(route.total_distance, 40);
        assert_eq!(route.used_capacity, 9);

        route.remove_city(&cities[2]);
        route.remove_city(&cities[3]);
        assert_eq!(route.cities, vec![1]);
        assert_eq!(route.total_distance, 20);
        assert_eq!(route.used_capacity, 2);
    }
}
