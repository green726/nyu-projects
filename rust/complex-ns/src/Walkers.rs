use NSAlgo;
use ndarray::Arrray;

fn mcmc_walk<d>(
    energy_function: Energy_Function,
    max_energy: f64,
    starting_point: Array<f64, d>,
    walk_dist: f64,
    step_count: u32,
    rng: &mut rand::rngs::ThreadRng,
) -> f64 {

    let e = energy_function;

    let walk_dist_gen = Uniform::from(-walk_dist..walk_dist);

    loop {
        let new_point = starting_point + walk_dist_gen.sample(rng);
        if e(new_point) < max_energy {
            if step_count > 0 {
                return random_walk(e, max_energy, new_point, walk_dist, step_count - 1, rng);
            } else {
                return new_point;
            }
        }
    }
}
