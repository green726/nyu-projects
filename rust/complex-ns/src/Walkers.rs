use rand::{distributions::Uniform, prelude::Distribution};

use crate::NSAlgo;

fn mcmc_walk(
    energy_function: NSAlgo::EnergyFunction,
    max_energy: f64,
    starting_point: Vec<f64>,
    walk_dist: f64,
    step_count: u32,
    rng: &mut rand::rngs::ThreadRng,
) -> f64 {
    let e = energy_function;

    for p in starting_point.iter() {
        let walk_dist_gen = Uniform::from(-walk_dist..walk_dist);

        for i in 0..step_count {
            
        }
    }
    return 0.0;
}
