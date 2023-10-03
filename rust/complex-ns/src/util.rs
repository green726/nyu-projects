use std::sync::RwLock;

use rand::{distributions::Uniform, prelude::Distribution};

pub fn random_point(state: &Vec<RwLock<Vec<f64>>>, rng: &mut rand::rngs::ThreadRng) -> Vec<f64> {
    let state_idx_gen = Uniform::from(0..state.len());
    let state_idx = state_idx_gen.sample(rng);
    return state[state_idx].read().unwrap().clone();
}

pub fn states_populate(
    dimensions: usize,
    count: usize,
    range: std::ops::Range<f64>,
    rng: &mut rand::rngs::ThreadRng,
) -> Vec<RwLock<Vec<f64>>> {
    let mut states: Vec<RwLock<Vec<f64>>> = Vec::new();
    let dist = Uniform::from(range);

    for _ in 0..count {
        let mut sample: Vec<f64> = Vec::new();
        for _ in 0..dimensions {
            sample.push(dist.sample(rng));
        }
        states.push(RwLock::new(sample));
    }

    println!("Initial States Have Been Uniformly Populated");

    return states;
}


