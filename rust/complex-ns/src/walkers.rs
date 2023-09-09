use rand::{distributions::Uniform, prelude::Distribution};

use crate::ns_algo;

pub struct WalkerConfig {
    pub step_dist: f64,
    pub step_count: u32,
}

impl WalkerConfig {
    pub fn new(step_dist: f64, step_count: u32) -> WalkerConfig {
        return WalkerConfig {
            step_dist,
            step_count,
        };
    }
}

pub fn mcmc_walk(
    energy_function: ns_algo::EnergyFunction,
    max_energy: f64,
    starting_state: Vec<f64>,
    walk_dist: f64,
    step_count: u32,
    rng: &mut rand::rngs::ThreadRng,
) -> Vec<f64> {
    let e = energy_function;
    let walk_dist_gen = Uniform::from(-walk_dist..walk_dist);

    let mut new_state: Vec<f64> = starting_state.clone();

    println!("Oh boy - starting mcmc walk");

    for _ in 0..step_count {
        let mut temp_state = new_state.clone();
        while e(temp_state.clone()) >= max_energy {
            for c in new_state.iter() {
                let new_c = c + walk_dist_gen.sample(rng);
                temp_state.push(new_c);
            }
        }
        new_state = temp_state;
    }

    println!("Yay! mcmc walk done!");
    return new_state;
}
