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

fn get_solution(cities: &[City], distance_table: &Distance) -> Vec<Route> {
    let mut route1 = Route::new();
    route1.add_city(&cities[1], 0);
    route1.add_city(&cities[2], 1);
    route1.add_city(&cities[3], 2);
    route1.updade_distance(distance_table);

    let mut route2 = Route::new();
    route2.add_city(&cities[4], 3);
    route2.updade_distance(distance_table);

    vec![route1, route2]
}

/**
 * Challenge yourself with this classic NP-Hard optimization problem !
 **/
fn main() {
    let (capacity, cities) = load_input();

    let distance_table = Distance::new(&cities);

    eprintln!("{:?}", distance_table);

    let solution = get_solution(&cities, &distance_table);

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
