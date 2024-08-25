use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::DefaultHasher;
use std::time::Instant;

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::city::City;
use crate::distance::Distance;
use crate::solution::Solution;

fn get_naive_solution(cities: &[City], capacity: u16, distance_table: &Distance) -> Solution {
    let mut ans: Vec<usize> = Vec::new();
    let mut total_distance: u32 = 0;

    let mut used = [false; 200];
    used[0] = true;

    let mut current_city = 0;
    let mut remaining_capacity = capacity;
    while ans.len() < cities.len() - 1 {
        let mut closest_index = 255;
        let mut closest_distance = u32::MAX;
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
        hashcode: 0,
    }
}

fn evolve(
    naive_ans: Solution,
    cities: &[City],
    capacity: u16,
    distance_table: &Distance,
) -> Solution {
    let mut visited_states: HashSet<u64> = HashSet::new();
    let mut hasher = DefaultHasher::new();

    let mut population = vec![naive_ans];

    let mut rng = thread_rng();
    let max_population = 400;
    let selected_population = 200;
    // let crossover_rate = 0.8;
    let mutation_rate = 0.02;
    let allele_length = cities.len() - 1;
    let duration_ms = 9590;

    let start_time = Instant::now();

    for _ in 1..max_population {
        let mut v: Vec<usize> = (1..cities.len()).collect();
        v.shuffle(&mut rng);

        let mut solution = Solution {
            routes: v,
            score: 0,
            hashcode: 0,
        };
        evaluate_solution(&mut solution, cities, capacity, distance_table);
        solution.calculate_hash(&mut hasher);

        if visited_states.contains(&solution.hashcode) {
            continue;
        }
        population.push(solution);
    }

    eprintln!(
        "Starting evolution after {} ms",
        start_time.elapsed().as_millis()
    );

    let mut generation = 0;
    while start_time.elapsed().as_millis() < duration_ms {
        // selection
        population.sort_by(|sol_a, sol_b| sol_a.score.cmp(&sol_b.score));

        // eprintln!("Best @{}: {}", generation, &population[0].score);

        population.truncate(selected_population);

        // crossover
        while population.len() < max_population {
            let parent1 = rng.gen_range(0..selected_population);
            let parent2 = rng.gen_range(0..selected_population);
            if parent1 == parent2 {
                continue;
            }

            let mut start = rng.gen_range(0..allele_length);
            let mut stop = rng.gen_range(0..allele_length);

            match start.cmp(&stop) {
                Ordering::Greater => std::mem::swap(&mut start, &mut stop),
                Ordering::Less => (),
                Ordering::Equal => continue,
            }

            let c1 = order_crossover(
                &population[parent1].routes,
                &population[parent2].routes,
                start,
                stop,
            );
            let c2 = order_crossover(
                &population[parent2].routes,
                &population[parent1].routes,
                start,
                stop,
            );

            let mut solution = Solution {
                routes: c1,
                score: 0,
                hashcode: 0,
            };
            evaluate_solution(&mut solution, cities, capacity, distance_table);
            solution.calculate_hash(&mut hasher);

            if visited_states.contains(&solution.hashcode) {
                continue;
            }
            visited_states.insert(solution.hashcode);
            population.push(solution);

            let mut solution = Solution {
                routes: c2,
                score: 0,
                hashcode: 0,
            };
            evaluate_solution(&mut solution, cities, capacity, distance_table);
            solution.calculate_hash(&mut hasher);

            if visited_states.contains(&solution.hashcode) {
                continue;
            }
            visited_states.insert(solution.hashcode);
            population.push(solution);
        }

        // mutation
        for i in 0..selected_population {
            let random_float: f64 = rng.gen();
            if random_float < mutation_rate {
                let a = rng.gen_range(0..allele_length);
                let b = rng.gen_range(0..allele_length);

                let mut new_route = population[i].routes.clone();
                new_route.swap(a, b);

                let mut solution = Solution {
                    routes: new_route,
                    score: 0,
                    hashcode: 0,
                };
                evaluate_solution(&mut solution, cities, capacity, distance_table);
                visited_states.insert(solution.hashcode);
                population.push(solution);
            }
        }

        generation += 1;
    }

    eprintln!(
        "Ending evolution after {} ms ({} generations)",
        start_time.elapsed().as_millis(),
        generation
    );

    population.sort_by(|sol_a, sol_b| sol_a.score.cmp(&sol_b.score));

    population.first().unwrap().to_owned()
}

fn evaluate_solution(
    solution: &mut Solution,
    cities: &[City],
    capacity: u16,
    distance_table: &Distance,
) {
    solution.score = 0;
    let mut current_city: usize = 0;
    let mut remaining_capacity = capacity;
    for city_id in solution.routes.iter() {
        if remaining_capacity >= cities[*city_id].get_demand() {
            solution.score += distance_table.get(current_city, *city_id);
            current_city = *city_id;
            remaining_capacity -= cities[*city_id].get_demand();
        } else {
            solution.score += distance_table.get(current_city, 0) + distance_table.get(0, *city_id);
            remaining_capacity = capacity - cities[*city_id].get_demand();
            current_city = *city_id;
        }
    }
    solution.score += distance_table.get(current_city, 0);
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

fn bruteforce_solution(cities: &[City], capacity: u16, distance_table: &Distance) -> Solution {
    let n = cities.len();
    let mut best_solution = Solution {
        routes: vec![],
        score: u32::MAX,
        hashcode: 0,
    };
    for new_route in (1..n).permutations(n - 1) {
        let mut solution = Solution {
            routes: new_route,
            score: 0,
            hashcode: 0,
        };
        evaluate_solution(&mut solution, cities, capacity, distance_table);
        if solution.score < best_solution.score {
            best_solution = solution;
        }
    }

    best_solution
}

pub fn get_solution(cities: &[City], capacity: u16, distance_table: &Distance) -> Solution {
    if cities.len() < 10 {
        bruteforce_solution(cities, capacity, distance_table)
    } else {
        let naive_ans = get_naive_solution(cities, capacity, distance_table);
        evolve(naive_ans, cities, capacity, distance_table)
    }
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
