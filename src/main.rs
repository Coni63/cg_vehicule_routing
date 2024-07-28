mod city;
mod distance;
mod route;

use city::City;
use distance::Distance;
use route::Route;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

pub fn load_input() -> (u16, Vec<City>) {
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // The number of customers

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let capacity = parse_input!(input_line, u16); // The capacity of the vehicles

    let mut cities: Vec<City> = Vec::new();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let index = parse_input!(inputs[0], usize); // The index of the customer (0 is the depot)
        let x = parse_input!(inputs[1], i16); // The x coordinate of the customer
        let y = parse_input!(inputs[2], i16); // The y coordinate of the customer
        let demand = parse_input!(inputs[3], u16); // The demand

        cities.push(City::new(index, x, y, demand));
    }

    (capacity, cities)
}

fn get_solution<'a>(
    cities: &[City],
    capacity: u16,
    distance_table: &'a Distance,
) -> Vec<Route<'a>> {
    let mut ans: Vec<Route<'a>> = Vec::new();
    // let mut route1 = Route::new(distance_table);
    // route1.add_city(&cities[1], 0);
    // route1.add_city(&cities[2], 1);
    // route1.add_city(&cities[3], 2);

    // let mut route2 = Route::new(distance_table);
    // route2.add_city(&cities[4], 3);

    // vec![route1, route2]

    let mut used = [false; 200];
    used[0] = true;

    loop {
        let mut route = Route::new(distance_table);
        let mut current_city = 0;
        loop {
            let mut closest_index = 255;
            let mut closest_distance = f32::MAX;
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

/**
 * Challenge yourself with this classic NP-Hard optimization problem !
 **/
fn main() {
    let (capacity, cities) = load_input();

    let distance_table = Distance::new(&cities);

    eprintln!("{:?}", distance_table);

    let solution = get_solution(&cities, capacity, &distance_table);

    let mut final_distance = 0.0;
    let mut final_string: Vec<String> = Vec::new();
    for route in solution.iter() {
        eprintln!("{:?}", route);
        final_distance += route.get_total_distance();
        final_string.push(route.to_string());
    }

    println!("{}", final_string.join(";"));
    println!("{}", final_distance);
}
