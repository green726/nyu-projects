use crate::{
    ns_algo::{NSConfig, EndConditionConfig},
    post_processing::{GraphResults, SpitResults},
    util::states_populate,
};

mod ns_algo;
mod plotting;
mod post_processing;
mod util;
mod walkers;

fn energy_function_1d(state: Vec<f64>) -> f64 {
    return state[0].powi(2);
}

fn gaussian_1d(center: f64, sd: f64, a: f64, state: Vec<f64>) -> f64 {
    let x_term = (state[0] - center).powi(2) / (2.0 * sd.powi(2));
    return a * (-(x_term)).exp();
}

fn gaussian_2d(center: [f64; 2], sd: [f64; 2], a: f64, state: Vec<f64>) -> f64 {
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
    return gaussian_1d(10.0, 4.0, 5.0, state.clone()) /* + gaussian_1d(-4.0, 3.0, 3.0, state.clone()) + gaussian_1d(2.0, 1.0, 2.0, state.clone()) */;
    //range: [-5, 10]
}

fn sinusoidal_product_4d(state: Vec<f64>) -> f64 {
    return state[0].sin(); /*  * state[1].sin() * state[2].sin() * state[3].sin(); */
}

fn harmonic_potential(state: Vec<f64>) -> f64 {
    return (0.5)
        * (state[0].powi(2)
            + state[1].powi(2)
            + state[2].powi(2)
            + state[3].powi(2)
            + state[4].powi(2)
            + state[5].powi(2)
            + state[6].powi(2)
            + state[7].powi(2));
}

fn harmonic_potential_1d(state: Vec<f64>) -> f64 {
    return (0.5) * (state[0].powi(2));
}

fn harmonic_potential_3d(state: Vec<f64>) -> f64 {
    return (0.5) * (state[0].powi(2) + state[1].powi(2) + state[2].powi(2));
}

fn harmonic_potential_2d(state: Vec<f64>) -> f64 {
    return (0.5) * (state[0].powi(2) + state[1].powi(2));
}

fn harmonic_potential_4d(state: Vec<f64>) -> f64 {
    return (0.5) * (state[0].powi(2) + state[1].powi(2) + state[2].powi(2) + state[3].powi(2));
}

fn harmonic_potential_5d(state: Vec<f64>) -> f64 {
    return (0.5)
        * (state[0].powi(2)
            + state[1].powi(2)
            + state[2].powi(2)
            + state[3].powi(2)
            + state[4].powi(2));
}

fn lj(state: Vec<f64>) -> f64 {
    let mut energy = 0.0;

    let mut particle_a: Vec<f64> = Vec::new();
    // println!("state: {:?}", state);
    for i in 0..state.len() {
        //Collect the three coords of each particle
        particle_a.push(state[i]);

        //Calculate the LJ potential based on this particle's three coords
        if (i + 1) % 3 == 0 {
            let mut particle_b: Vec<f64> = Vec::new();
            // println!("particle A: {:?}", particle_a);

            for j in i + 1..state.len() {
                particle_b.push(state[j]);

                if (j + 1) % 3 == 0 {
                    // println!("particle B: {:?}", particle_b);
                    let r = ((particle_a[0] - particle_b[0]).powi(2)
                        + (particle_a[1] - particle_b[1]).powi(2)
                        + (particle_a[2] - particle_b[2]).powi(2))
                    .sqrt();
                    energy += 4.0 * (1.0 / r.powi(12) - 1.0 / r.powi(6));
                    particle_b.clear();
                }
            }
            particle_a.clear();
        }
    }
    // panic!();
    // println!("energy: {}", energy);
    // panic!();
    return energy;
}

fn main() {
    let mut rng = rand::thread_rng();

    //LJ 4 cluster
    let n = 1000;
    let d = 15;
    let config = NSConfig::new(
        lj,
        states_populate(d, n, -1.0..1.0, &mut rng),
        n,
        2000000,
        true,
        walkers::WalkerConfig::new(0.001, 1),
        d,
        EndConditionConfig::new(ns_algo::EndCondition::Avg, 50, 0.00000001)
    );

    //harmonic potential 1d
    // let n = 1000;
    // let config = NSConfig::new(
    //     harmonic_potential_1d,
    //     states_populate(1, n, -2.0..2.0, &mut rng),
    //     n,
    //     100000,
    //     false,
    //     walkers::WalkerConfig::new(0.0000001, 1),
    // );

    // harmonic potential 2d
    // let n = 6000;
    // let config = NSConfig::new(
    //     harmonic_potential_2d,
    //     states_populate(2, n, 0.0..30.0, &mut rng),
    //     n,
    //     75000,
    //     false,
    //     walkers::WalkerConfig::new(0.00001, 1),
    //     2,
    // );

    //harmonic potential 3d
    // let n = 4000;
    // let config = NSConfig::new(
    //     harmonic_potential_3d,
    //     states_populate(3, n, 0.0..60.0, &mut rng),
    //     n,
    //     400000,
    //     false,
    //     walkers::WalkerConfig::new(0.001, 2),
    // );

    //harmonic potential 4d
    // let n = 2000;
    // let config = NSConfig::new(
    //     harmonic_potential_4d,
    //     states_populate(4, n, 0.0..20.0, &mut rng),
    //     n,
    //     100000,
    //     false,
    //     walkers::WalkerConfig::new(0.001, 1),
    // );

    //harmonic potential 5d
    // let n = 2000;
    // let config = NSConfig::new(
    //     harmonic_potential_5d,
    //     states_populate(5, n, 0.0..20.0, &mut rng),
    //     n,
    //     300000,
    //     false,
    //     walkers::WalkerConfig::new(0.00001, 1),
    // );

    // 1d gaussian superposition
    // let n = 1000;
    // let config = NSConfig::new(
    //     gaussian_superposition_1d,
    //     states_populate(1, n, -3.0..6.0, &mut rng),
    //     n,
    //     10000,
    //     false,
    //     walkers::WalkerConfig::new(0.0001, 3),
    // );

    //4d sinusoidal
    // let n = 1000;
    // let config = NSConfig::new(
    //     sinusoidal_product_4d,
    //     states_populate(1, n, -3.0..-0.1, &mut rng),
    //     n,
    //     10000,
    //     false,
    //     walkers::WalkerConfig::new(0.001, 3),
    // );

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

    // let pp_result = post_processing::post_process(ns_result);
    // pp_result.graph_free_energies();
    // pp_result.graph_volume();
    // pp_result.graph_density_of_states();
    // pp_result.graph_ge_de();
    // pp_result.spit_ge_de();
    // pp_result.spit_max_energy();
}
