use std::time::Instant;

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::city::City;
use crate::distance::Distance;
use crate::route::Route;

#[derive(Clone, PartialEq, Debug)]
enum Strategy {
    InnerSwap,
    // OuterSwap,
    // Transfert,
}

impl Strategy {
    fn all_variants() -> &'static [Strategy] {
        static VARIANTS: &[Strategy] = &[
            Strategy::InnerSwap,
            // Strategy::OuterSwap,
            // Strategy::Transfert,
        ];
        VARIANTS
    }

    // Randomly pick one of the enum variants
    fn random(rng: &mut ThreadRng) -> Strategy {
        let variants = Strategy::all_variants();
        let idx = rng.gen_range(0..variants.len());
        variants[idx].clone()
    }
}

fn get_naive_solution<'a>(
    cities: &[City],
    capacity: u16,
    distance_table: &'a Distance,
) -> Vec<Route<'a>> {
    let mut ans: Vec<Route<'a>> = Vec::new();

    let mut used = [false; 200];
    used[0] = true;

    loop {
        let mut route = Route::new(distance_table);
        let mut current_city = 0;
        loop {
            let mut closest_index = 255;
            let mut closest_distance = u16::MAX;
            for (i, city) in cities.iter().enumerate() {
                let d = distance_table.get(current_city, i);
                if !used[i] && (d < closest_distance) && route.can_accept(city, capacity) {
                    closest_distance = d;
                    closest_index = i;
                }
            }

            if closest_index == 255 {
                break;
            }

            used[closest_index] = true;
            route.add_city(&cities[closest_index], 255);
            current_city = closest_index;
        }

        if route.is_empty() {
            break;
        }

        ans.push(route);
    }

    ans
}

fn get_score(solution: &[Route]) -> u16 {
    solution.iter().map(|x| x.get_total_distance()).sum()
}

fn evolve<'a>(base_solution: Vec<Route<'a>>, cities: &[City]) -> Vec<Route<'a>> {
    let mut rng = rand::thread_rng();
    let now = Instant::now();

    eprintln!("{:?}", base_solution);

    let mut best_solution = base_solution.clone();
    let mut active_solution = base_solution.clone();
    let mut current_solution = base_solution.clone();

    let mut best_score: u16 = get_score(&base_solution);
    let mut active_score: u16 = get_score(&active_solution);
    let mut current_score: u16 = get_score(&current_solution);

    let mut loop_ = 0;
    while now.elapsed().as_millis() < 1000 {
        let strategy = if current_solution.len() > 1 {
            Strategy::InnerSwap
        } else {
            Strategy::random(&mut rng)
        };

        match strategy {
            Strategy::InnerSwap => {
                let idx_route_impacted = rng.gen_range(0..current_solution.len());
                let route = current_solution.get_mut(idx_route_impacted).unwrap();
                let idx1 = rng.gen_range(0..route.len());
                let idx2 = rng.gen_range(0..route.len());
                route.inner_swap_city(idx1, idx2);
            } // Strategy::OuterSwap => {}
              // Strategy::Transfert => {}
        }

        current_score = get_score(&current_solution);
        if current_score < best_score {
            // eprintln!("New Best @{} - {:?}", best_score, current_solution);
            best_score = current_score;
            active_score = current_score;
            best_solution = current_solution.clone();
            active_solution = current_solution.clone();
        }

        if current_score > (1.05 * (active_score as f32)) as u16 {
            current_solution = active_solution.clone();
        }

        loop_ += 1;
    }

    eprintln!("{} loops", loop_);

    best_solution
}

pub fn get_solution<'a>(
    cities: &[City],
    capacity: u16,
    distance_table: &'a Distance,
) -> Vec<Route<'a>> {
    let naive_ans = get_naive_solution(cities, capacity, distance_table);

    let ans = evolve(naive_ans, cities);

    ans
}
