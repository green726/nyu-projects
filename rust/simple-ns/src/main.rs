use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;

fn fourth_degree_energy_func(x: f64) -> f64 {
    return 2.0 * x.powi(4) - (3.0 * x.powi(3)) - (7.0 * x.powi(2)) + (2.0 * x) + 1.0;
}

fn main() {
    let mut rng = rand::thread_rng();
    let k = 100;
    let prior_points = populate_vec(100, f64::from(-(k/2)), f64::from(k/2), &mut rng);
    let result = nested_sampling(&fourth_degree_energy_func, 1000, prior_points, false, &mut rng);
    println!("{:?}", result);
}

//function to populate a vec with k random numbers constrained with a min and max 
fn populate_vec(k: i32, min: f64, max: f64, rng: &mut rand::rngs::ThreadRng) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::new();
    let uniform = Uniform::from(min..max);

    for _ in 0..k {
        vec.push(uniform.sample(rng));
    }

    return vec;
}

fn max_energy(energy_function: &dyn Fn(f64) -> f64, replicas: Vec<f64>) -> (f64, usize) {
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

fn random_walk(energy_function: &dyn Fn(f64) -> f64, max_energy: f64, starting_point: f64, walk_dist: f64, rng: &mut rand::rngs::ThreadRng) -> f64 {
    let e = energy_function;

    let walk_dist_gen = Uniform::from(-walk_dist..walk_dist);

    loop {
        let new_point = starting_point + walk_dist_gen.sample(rng);
        if e(new_point) < max_energy {
            return new_point;
        }
    }
}


fn nested_sampling(energy_function: &dyn Fn(f64) -> f64, iterations: i32, prior_points: Vec<f64>, debug: bool, rng: &mut rand::rngs::ThreadRng) -> Vec<f64>{
    let e = energy_function;

    let mut max_energies: Vec<f64> = Vec::new();
    let mut replicas = prior_points;

    for _ in 0..iterations {
        // Find the max energy
        let (max_energy, max_energy_idx) = max_energy(e, replicas.clone());
        max_energies.push(max_energy);

        // Remove the max energy replica
        replicas.remove(max_energy_idx);

        // Add a new replica
        let new_replica = random_walk(e, max_energy, replicas.choose(rng).unwrap().clone(), 1.0, rng);
        replicas.push(new_replica);
    }

    return max_energies;
}
