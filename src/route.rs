use crate::{city::City, distance::Distance};
use std::fmt;

pub struct Route<'a> {
    cities: Vec<usize>,
    total_distance: f32,
    used_capacity: u16,
    distances: &'a Distance,
}

impl<'a> Route<'a> {
    pub fn new(distances: &'a Distance) -> Route {
        Route {
            cities: Vec::new(),
            total_distance: 0.0,
            used_capacity: 0,
            distances,
        }
    }

    pub fn add_city(&mut self, city: &City, position: usize) {
        if position >= self.cities.len() {
            self.cities.push(city.get_index());
        } else {
            self.cities.insert(position, city.get_index());
        }
        self.used_capacity += city.get_capacity();
        self.updade_distance();
    }

    pub fn remove_city(&mut self, city: &City) {
        if let Some(index) = self.cities.iter().position(|x| *x == city.get_index()) {
            self.cities.remove(index);
            self.used_capacity -= city.get_capacity();
        }
        self.updade_distance();
    }

    fn updade_distance(&mut self) {
        let n = self.cities.len() - 1;
        self.total_distance = 0.0;
        for i in 0..n {
            self.total_distance += self.distances.get(self.cities[i], self.cities[i + 1]);
        }
        self.total_distance += self.distances.get(0, self.cities[0]);
        self.total_distance += self.distances.get(self.cities[n], 0);
    }

    pub fn get_capacity(&self) -> u16 {
        self.used_capacity
    }

    pub fn get_total_distance(&self) -> f32 {
        self.total_distance
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
