use crate::{ns_algo::NSConfig, post_processing::GraphResults, util::states_populate};

mod ns_algo;
mod plotting;
mod post_processing;
mod util;
mod walkers;



fn energy_function_1d(state: Vec<f64>) -> f64 {
    return state[0].powi(2);
}

fn gaussian_1d(center: f64, sd: f64, a: f64, state: Vec<f64>) -> f64{
     let x_term = (state[0] - center).powi(2) / (2.0 * sd.powi(2));
    return a * (-(x_term)).exp();
}

fn gaussian_2d(center: [f64; 2], sd: [f64; 2], a: f64, state: Vec<f64>) -> f64{
     let x_term = (state[0] - center[0]).powi(2) / (2.0 * sd[0].powi(2));
    let y_term = (state[1] - center[1]).powi(2) / (2.0 * sd[1].powi(2));
    return a * (-(x_term + y_term)).exp();
}

fn gaussian_superposition_2d(state: Vec<f64>) -> f64 {
    let gauss_1 = gaussian_2d([-10.0, -10.0], [1.0, 1.0], 1.0, state.clone());
    let gauss_2 = gaussian_2d([1.0, -9.0], [19.0, 6.5], 19.0, state.clone());
    let gauss_3 = gaussian_2d([18.0, 39.0], [3.0, 1.2], 8.0, state.clone());
    return gauss_1 + gauss_2 + gauss_3;
}

fn gaussian_superposition_1d(state: Vec<f64>) -> f64 {
    return gaussian_1d(10.0, 4.0, 5.0, state.clone()) + gaussian_1d(-4.0, 3.0, 3.0, state.clone()) + gaussian_1d(2.0, 1.0, 2.0, state.clone());
    //range: [-5, 10]
}

fn sinusoidal_product_4d(state: Vec<f64>) -> f64 {
    return state[0].sin() * state[1].sin() * state[2].sin() * state[3].sin();
}


fn main() {
    let mut rng = rand::thread_rng();


    // // 1d gaussian superposition
    // let n = 1000;
    // let config = NSConfig::new(
    //     gaussian_superposition_1d,
    //     states_populate(1, n, -5.0..10.0, &mut rng),
    //     n,
    //     10000,
    //     false,
    //     walkers::WalkerConfig::new(0.0001, 3),
    // );
   
    //4d sinusoidal
    let n = 100;
    let config = NSConfig::new(
        sinusoidal_product_4d,
        states_populate(4, n, -5.0..5.0, &mut rng),
        n,
        50000,
        false,
        walkers::WalkerConfig::new(0.001, 3),
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
    pp_result.graph_density_of_states();
}
