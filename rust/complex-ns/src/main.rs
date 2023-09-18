use crate::{ns_algo::NSConfig, util::states_populate};

mod ns_algo;
mod walkers;
mod util;


fn energy_function(state: Vec<f64>) -> f64 {
    return (state[0] + state[1]).powi(2);
}

fn main() {
    let mut rng = rand::thread_rng();

    let config = NSConfig::new(
        energy_function,
        states_populate(2, 100, -10.0..10.0, &mut rng),
        100,
        false,
        walkers::WalkerConfig::new(0.000001, 3),
    );

    let result = ns_algo::algo(config);

    println!("max e: {}", result.max_energies[0]);
}


