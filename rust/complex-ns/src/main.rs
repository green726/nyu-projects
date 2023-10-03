use crate::{ns_algo::NSConfig, post_processing::GraphResults, util::states_populate};

mod ns_algo;
mod plotting;
mod post_processing;
mod util;
mod walkers;

fn energy_function(state: Vec<f64>) -> f64 {
    return (state[0] + state[1]).powi(2);
}

fn energy_function_1d(state: Vec<f64>) -> f64 {
    return state[0].powi(2);
}

fn energy_function_complex(state: Vec<f64>) -> f64 {
    return state[0].sin() - (2.0 * state[1].powi(3));
}

fn energy_function_6d_complex(state: Vec<f64>) -> f64 {
    return state[0].sin() - (6.1 * state[1].powi(2)) + (1.1 * state[2]).powi(3) - (2.8 * state[3])
        + (4.1 * state[4].cos())
        + (state[5].powi(2) * 0.3);
}

fn main() {
    let mut rng = rand::thread_rng();

    // let config = NSConfig::new(
    //     energy_function_complex,
    //     states_populate(6, 1000, -100.0..100.0, &mut rng),
    //     1000,
    //     100,
    //     false,
    //     walkers::WalkerConfig::new(0.0001, 4),
    // );


    let n = 100;
    let config = NSConfig::new(
        energy_function_complex,
        states_populate(2, n, -100.0..100.0, &mut rng),
        n,
        10000,
        false,
        walkers::WalkerConfig::new(0.0001, 3),
    );

    // let config = NSConfig::new(
    //     energy_function_1d,
    //     states_populate(1, n, -10.0..10.0, &mut rng),
    //     n, //n
    //     1000, //i
    //     false,
    //     walkers::WalkerConfig::new(0.001, 3),
    // );

    let ns_result = ns_algo::algo(config);

    println!("min e: {}", ns_result.max_energies.last().unwrap());

    let pp_result = post_processing::post_process(ns_result);
    pp_result.graph_free_energies();
    pp_result.graph_volume();
}
