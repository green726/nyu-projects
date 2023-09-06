mod energy_funcs;
mod util;

use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use util::max_energy;
use util::populate_vec;
use util::MinEnergy;
use util::NsOutput;

use plotpy::{Curve, Plot};

fn main() {
    let mut rng = rand::thread_rng();
    let ns_result = nested_sampling(
        &energy_funcs::complex_energy_func,
        1000,
        100,
        true,
        &mut rng,
    );

    println!("state count:");

    plot_data(
        Plot::new(),
        ns_result.max_energies.clone(),
        ns_result.state_counts.clone(),
        "Max Energy vs. # Of States",
        "Max Energy",
        "# Of States",
    );
}

fn plot_data(
    mut plot: Plot,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    title: &str,
    x_label: &str,
    y_label: &str,
) {
    // configure curve
    let mut curve = Curve::new();
    curve.set_line_width(2.0);

    // draw curve
    curve.draw(&x_values, &y_values);

    // add curve to plot
    plot.set_title(title);
    plot.add(&curve).grid_labels_legend(x_label, y_label);

    // save figure
    let _ = plot.save_and_show("./plot.svg");
}

fn random_walk(
    energy_function: &dyn Fn(f64) -> f64,
    max_energy: f64,
    starting_point: f64,
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

fn nested_sampling_algo(
    energy_function: &dyn Fn(f64) -> f64,
    iterations: i32,
    prior_points: Vec<f64>,
    debug: bool,
    rng: &mut rand::rngs::ThreadRng,
) -> (Vec<f64>, MinEnergy) {
    let e = energy_function;

    let mut max_energies: Vec<f64> = Vec::new();
    let mut replicas = prior_points;

    let mut min_energy = MinEnergy {
        energy: -1.0,
        state: -1.0,
        replica_idx: 1,
    };

    for n in 0..iterations {
        // Find the max energy
        let (max_energy, max_energy_idx) = max_energy(e, replicas.clone());
        max_energies.push(max_energy);

        if n == iterations {
            min_energy = MinEnergy {
                energy: max_energy,
                state: replicas[max_energy_idx],
                replica_idx: max_energy_idx,
            };
        }

        // Remove the max energy replica
        replicas.remove(max_energy_idx);

        let new_replica = random_walk(
            e,
            max_energy,
            replicas.choose(rng).unwrap().clone(),
            0.001,
            6,
            rng,
        );
        replicas.push(new_replica);

        for i in 0..replicas.len() {
            replicas[i] = random_walk(e, max_energy, replicas[i].clone(), 0.001, 6, rng);
        }
    }

    return (max_energies, min_energy);
}

fn avg_energy(ns_result: Vec<f64>) -> Vec<f64> {
    let mut avg_energy: Vec<f64> = Vec::new();

    for i in 0..ns_result.len() {
        if i != ns_result.len() - 1 {
            avg_energy.push((ns_result[i] + ns_result[i + 1]) / 2.0);
        } else {
            avg_energy.push(ns_result[i]);
        }
    }

    return avg_energy;
}

fn free_energy(
    energy_function: &dyn Fn(f64) -> f64,
    iterations: i32,
    k: i32,
    debug: bool,
    max_energies: Vec<f64>,
    rng: &mut rand::rngs::ThreadRng,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut density_of_states: Vec<f64> = Vec::new();
    let mut free_energies: Vec<f64> = Vec::new();

    for i in 0..max_energies.len() {
        let k_f = k as f64;
        let density_of_state = (1.0 / (k_f + 1.0)) * (k_f / (k_f + 1.0)).powf(i as f64);
        density_of_states.push(density_of_state);

        let free_energy = -1.0 * density_of_state.ln();
        free_energies.push(free_energy);
    }

    let avg_energies = avg_energy(max_energies.clone());
    if debug {
        plot_data(
            Plot::new(),
            avg_energies.clone(),
            free_energies.clone(),
            "Free Energy vs. Energy",
            "Energy",
            "Free Energy",
        );
    }

    return (free_energies, density_of_states, avg_energies);
}

fn nested_sampling(
    energy_function: &dyn Fn(f64) -> f64,
    iterations: i32,
    k: i32,
    debug: bool,
    rng: &mut rand::rngs::ThreadRng,
) -> NsOutput {
    let prior_points = populate_vec(k, f64::from(-(k / 2)), f64::from(k / 2), rng);
    let (max_energies, min_energy) =
        nested_sampling_algo(energy_function, iterations, prior_points, debug, rng);

    let (free_energies, density_of_states, avg_energies) = free_energy(
        energy_function,
        iterations,
        k,
        debug,
        max_energies.clone(),
        rng,
    );

    let mut state_counts: Vec<f64> = Vec::new();

    for i in 0..max_energies.len() {
        state_counts.push(density_of_states[i] * (max_energies[i] - min_energy.energy))
    }

    return NsOutput {
        max_energies: max_energies,
        free_energies: free_energies,
        density_of_states: density_of_states,
        min_energy: min_energy,
        avg_energies: avg_energies,
        state_counts: state_counts,
    };
}
