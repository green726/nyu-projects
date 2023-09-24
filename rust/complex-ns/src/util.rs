use rand::{distributions::Uniform, prelude::Distribution};

use plotly::{Plot, Scatter};

pub fn random_point(state: &Vec<Vec<f64>>, rng: &mut rand::rngs::ThreadRng) -> Vec<f64> {
    let state_idx_gen = Uniform::from(0..state.len());
    let state_idx = state_idx_gen.sample(rng);
    return state[state_idx].clone();
}


pub fn states_populate(dimensions: usize, count: usize, range: std::ops::Range<f64>, rng: &mut rand::rngs::ThreadRng) -> Vec<Vec<f64>> {
    let mut states: Vec<Vec<f64>> = Vec::new();
    let dist = Uniform::from(range);

    for _ in 0..count {
        let mut sample: Vec<f64> = Vec::new();
        for _ in 0..dimensions {
            sample.push(dist.sample(rng));
        }
        states.push(sample);
    }
    
    println!("Initial States Have Been Uniformly Populated");

    return states;
}

pub fn plot_data(x: Vec<f64>, y: Vec<f64>) {

} 
