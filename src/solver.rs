use std::time::Instant;

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::city::City;
use crate::distance::Distance;
use crate::solution::Solution;

fn get_naive_solution(cities: &[City], capacity: u16, distance_table: &Distance) -> Solution {
    let mut ans: Vec<usize> = Vec::new();
    let mut total_distance: u16 = 0;

    let mut used = [false; 200];
    used[0] = true;

    let mut current_city = 0;
    let mut remaining_capacity = capacity;
    while ans.len() < cities.len() - 1 {
        let mut closest_index = 255;
        let mut closest_distance = u16::MAX;
        for (i, city) in cities.iter().enumerate() {
            let d = distance_table.get(current_city, i);
            if !used[i] && (d < closest_distance) && remaining_capacity >= city.get_demand() {
                closest_distance = d;
                closest_index = i;
            }
        }
        eprintln!("{} {}", current_city, closest_index);
        if closest_index == 255 {
            total_distance += distance_table.get(current_city, 0);
            remaining_capacity = capacity;
            current_city = 0;
        } else {
            total_distance += closest_distance;
            remaining_capacity -= cities[closest_index].get_demand();
            current_city = closest_index;
            used[closest_index] = true;
            ans.push(closest_index);
        }
    }

    total_distance += distance_table.get(current_city, 0);

    Solution {
        routes: ans,
        score: total_distance,
    }
}

fn evolve(
    initial_solution: &Solution,
    cities: &[City],
    capacity: u16,
    distance_table: &Distance,
) -> Solution {
    initial_solution.clone()
}

pub fn get_solution(cities: &[City], capacity: u16, distance_table: &Distance) -> Solution {
    let naive_ans = get_naive_solution(cities, capacity, distance_table);
    let improved = evolve(&naive_ans, cities, capacity, distance_table);
    improved
}
