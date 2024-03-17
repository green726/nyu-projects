use std::sync::RwLock;

use rand;

use crate::{plotting, util, walkers};
use plotters::prelude::*;
use rayon::prelude::*;

pub type EnergyFunction = fn(Vec<f64>) -> f64;

pub struct NSConfig {
    energy_function: EnergyFunction,
    initial_state: Vec<RwLock<Vec<f64>>>,
    initial_len: usize,
    iterations: i32,
    //TODO: implement a struct containing various debug options (debug conf)
    debug: bool,
    rng: rand::rngs::ThreadRng,
    walker_config: walkers::WalkerConfig,
    dimensions: usize,
    end_condition: EndConditionConfig,
}

pub enum EndCondition {
    Avg,
    ValueTolerance,
}
pub struct EndConditionConfig {
    typ: EndCondition,
    avg_k: usize,
    avg_tolerance: f64,
}

impl EndConditionConfig {
    pub fn new(typ: EndCondition, avg_k: usize, avg_tolerance: f64) -> EndConditionConfig {
        return EndConditionConfig {
            typ,
            avg_k,
            avg_tolerance,
        };
    }
}

impl NSConfig {
    pub fn new(
        energy_function: EnergyFunction,
        initial_states: Vec<RwLock<Vec<f64>>>,
        initial_len: usize,
        iterations: i32,
        debug: bool,
        walker_config: walkers::WalkerConfig,
        dimensions: usize,
        end_condition: EndConditionConfig,
    ) -> NSConfig {
        if initial_states.len() < 2 {
            panic!("initial_states must have at least 2 replicas");
        }

        return NSConfig {
            energy_function,
            initial_state: initial_states,
            initial_len,
            iterations,
            debug,
            rng: rand::thread_rng(),
            walker_config,
            dimensions,
            end_condition,
        };
    }
}

pub struct MinEnergy {
    pub energy: f64,
    pub state: Vec<f64>,
    pub replica_idx: usize,
}

pub struct NSResult {
    pub max_energies: Vec<f64>,
    pub min_energy: MinEnergy,
    pub k: usize,
    pub iterations: i32,
    pub dimensions: usize,
}

fn max_energy(energy_function: EnergyFunction, state: &Vec<RwLock<Vec<f64>>>) -> (f64, usize) {
    let mut max_energy = 0.0;
    let mut max_energy_idx = 0;

    for i in 0..state.len() {
        let replica = &state[i];

        if i == 0 {
            max_energy = energy_function(replica.read().unwrap().clone());
            max_energy_idx = i;
            continue;
        }

        let energy = energy_function(replica.read().unwrap().clone());
        if energy > max_energy {
            max_energy = energy;
            max_energy_idx = i;
        }
    }

    return (max_energy, max_energy_idx);
}

pub fn algo(mut config: NSConfig) -> NSResult {
    let e = config.energy_function;
    let rng = &mut config.rng;
    let mut states = config.initial_state;

    let mut result = NSResult {
        max_energies: Vec::new(),
        min_energy: MinEnergy {
            energy: 0.0,
            state: Vec::new(),
            replica_idx: 0,
        },
        k: config.initial_len,
        iterations: config.iterations,
        dimensions: config.dimensions,
    };

    let plot_backend = plotting::create_plot_backend_gif("graphs/maxEnergy_vs_iteration.gif");
    let plot_drawing_area = plot_backend.into_drawing_area();

    let mut iterations_vec: Vec<f64> = Vec::new();

    'main_loop: {
        for n in 0..config.iterations {
            iterations_vec.push(n as f64);
            //find max energy
            let (max_energy, max_energy_idx) = max_energy(e, &states);
            result.max_energies.push(max_energy);

            if n == config.iterations - 1 {
                result.min_energy.energy = max_energy;
                result.min_energy.state = states[max_energy_idx].read().unwrap().clone();
                result.min_energy.replica_idx = max_energy_idx;
            }

            // Remove the max energy replica
            states.remove(max_energy_idx);

            match config.end_condition.typ {
                EndCondition::ValueTolerance => {
                    let tolerance = config.end_condition.avg_tolerance;
                    if max_energy < tolerance {
                        result.k = states.len();
                        result.iterations = n;
                        break 'main_loop;
                    }
                }
                EndCondition::Avg => 'avg_cond: {
                    let k = config.end_condition.avg_k;
                    let tolerance = config.end_condition.avg_tolerance;

                    if result.max_energies.len() < k + 1 {
                        break 'avg_cond;
                    }

                    let mut sum = 0.0;
                    for i in 0..k {
                        let de = result.max_energies[result.max_energies.len() - i - 1] - result.max_energies[result.max_energies.len() - i - 2];
                        sum += de;
                    }

                    let avg = sum / (k as f64);

                    if (avg - max_energy).abs() < tolerance {
                        result.k = states.len();
                        result.iterations = n;
                        break 'main_loop;
                    }
                }
                _ => {}
            }

            let new_replica = RwLock::new(walkers::mcmc_walk(
                e,
                max_energy,
                &RwLock::new(util::random_point(&states, rng)),
                config.walker_config.step_dist,
                config.walker_config.step_count,
                rng,
            ));
            states.push(new_replica);

            states.par_iter_mut().for_each(|replica| {
                let rng = &mut rand::thread_rng();
                *replica = RwLock::new(walkers::mcmc_walk(
                    e,
                    max_energy,
                    replica,
                    config.walker_config.step_dist,
                    config.walker_config.step_count,
                    rng,
                ));
            });

            // for i in 0..states.len() {
            //     states[i] = RwLock::new(walkers::mcmc_walk(e, max_energy, &states[i], config.walker_config.step_dist, config.walker_config.step_count, rng));
            // }

            if config.debug {
                // plotting::plot_data(
                //     "Max Energy",
                //     &plot_drawing_area,
                //     0.0..(n as f64),
                //     -result.max_energies[0]..(result.max_energies[0]),
                //     iterations_vec.clone(),
                //     result.max_energies.clone(),
                //     plotting::Scale::LinearLinear,
                // );
                println!("e_i: {}_{}", max_energy, n);
            }
        }
    }
    println!("final state: {:?}", states[0].read().unwrap().clone());

    return result;
}
