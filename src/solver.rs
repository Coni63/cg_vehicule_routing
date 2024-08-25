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

fn evaluate_solution(
    solution: &Solution,
    cities: &[City],
    capacity: u16,
    distance_table: &Distance,
) -> u16 {
    let mut total_distance = 0u16;
    let mut current_city: usize = 0;
    let mut remaining_capacity = capacity;
    for city_id in solution.routes.iter() {
        if remaining_capacity >= cities[*city_id].get_demand() {
            total_distance += distance_table.get(current_city, *city_id);
            current_city = *city_id;
            remaining_capacity -= cities[*city_id].get_demand();
        } else {
            total_distance += distance_table.get(current_city, 0) + distance_table.get(0, *city_id);
            remaining_capacity = capacity - cities[*city_id].get_demand();
            current_city = *city_id;
        }
    }
    total_distance += distance_table.get(current_city, 0);

    total_distance
}

fn order_crossover(parent1: &[usize], parent2: &[usize], a: usize, b: usize) -> Vec<usize> {
    let mut child = Vec::new();

    let mut num_value_left = 0;
    let mut hole_found: usize = 0;
    for value in parent1.iter() {
        let hole = is_hole(value, parent2, a, b);

        hole_found += hole as usize;

        if hole && hole_found == (b - a) {
            num_value_left = child.len();
            child.extend(std::iter::repeat(255).take(b - a));
        } else if !hole {
            child.push(*value);
        }
    }

    if num_value_left < a {
        child.rotate_right(a - num_value_left);
    } else if num_value_left > 0 {
        child.rotate_left(num_value_left - a);
    }

    child[a..b].clone_from_slice(&parent2[a..b]);

    child
}

fn is_hole(value: &usize, parent: &[usize], a: usize, b: usize) -> bool {
    for value_to_swap in parent.iter().skip(a).take(b - a) {
        if value == value_to_swap {
            return true;
        }
    }
    false
}

pub fn get_solution(cities: &[City], capacity: u16, distance_table: &Distance) -> Solution {
    let naive_ans = get_naive_solution(cities, capacity, distance_table);
    let improved = evolve(&naive_ans, cities, capacity, distance_table);
    improved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hole() {
        let parent = vec![1, 2, 3, 4, 5];
        //                     window = |    |
        assert!(!is_hole(&1, &parent, 1, 3));
        assert!(is_hole(&2, &parent, 1, 3));
        assert!(is_hole(&3, &parent, 1, 3));
        assert!(!is_hole(&4, &parent, 1, 3));
        assert!(!is_hole(&5, &parent, 1, 3));
    }

    #[test]
    fn test_crossover() {
        let parent1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let parent2 = vec![4, 5, 2, 1, 8, 7, 6, 9, 3];
        let a = 3;
        let b = 7;

        let c1 = order_crossover(&parent1, &parent2, a, b);
        let c2 = order_crossover(&parent2, &parent1, a, b);

        assert_eq!(c1, vec![3, 4, 5, 1, 8, 7, 6, 9, 2]);
        assert_eq!(c2, vec![2, 1, 8, 4, 5, 6, 7, 9, 3]);
    }

    #[test]
    fn test_crossover2() {
        let parent1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let parent2 = vec![8, 9, 1, 2, 3, 4, 5, 6, 7];
        let a = 7;
        let b = 9;

        let c1 = order_crossover(&parent1, &parent2, a, b);
        let c2 = order_crossover(&parent2, &parent1, a, b);

        assert_eq!(c1, vec![8, 9, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(c2, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_crossover3() {
        let parent1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let parent2 = vec![2, 4, 6, 8, 1, 3, 5, 7, 9];
        let a = 0;
        let b = 2;

        let c1 = order_crossover(&parent1, &parent2, a, b);
        let c2 = order_crossover(&parent2, &parent1, a, b);

        assert_eq!(c1, vec![2, 4, 5, 6, 7, 8, 9, 1, 3]);
        assert_eq!(c2, vec![1, 2, 3, 5, 7, 9, 4, 6, 8]);
    }
}
