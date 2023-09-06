use rand::distributions::{Distribution, Uniform};

pub struct NsOutput {
    pub max_energies: Vec<f64>,
    pub free_energies: Vec<f64>,
    pub density_of_states: Vec<f64>,
    pub state_counts: Vec<f64>,
    pub avg_energies: Vec<f64>,
    pub min_energy: MinEnergy,
}

pub struct MinEnergy {
    pub energy: f64,
    pub state: f64,
    pub replica_idx: usize,
}

//function to populate a vec with k random numbers constrained with a min and max 
pub fn populate_vec(k: i32, min: f64, max: f64, rng: &mut rand::rngs::ThreadRng) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::new();
    let uniform = Uniform::from(min..max);

    for _ in 0..k {
        vec.push(uniform.sample(rng));
    }

    return vec;
}

pub fn max_energy(energy_function: &dyn Fn(f64) -> f64, replicas: Vec<f64>) -> (f64, usize) {
    let e = energy_function;

    let mut max_energy = 0.0;
    let mut max_energy_idx: usize = 0;

    for i in 0..replicas.len() {

        if i == 0 {
            max_energy = e(replicas[i]);
            continue;
        }

        let energy = e(replicas[i]);
        if energy > max_energy {
            max_energy = energy;
            max_energy_idx = i;
        }
    }

    return (max_energy, max_energy_idx);
}
