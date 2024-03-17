use std::{io::Write};

use crate::{ns_algo, plotting};
use plotters::prelude::*;

pub trait GraphResults {
    fn graph_free_energies(&self);
    fn graph_density_of_states(&self);
    fn graph_volume(&self);
    fn graph_ge_de(&self);
}

//Spit as in "spit-out" (to a file)
pub trait SpitResults {
    // fn spit_free_energies(&self);
    // fn spit_density_of_states(&self);
    // fn spit_volume(&self);
    fn spit_ge_de(&self);
    fn spit_max_energy(&self);
}

pub struct PostProcessingResult {
    pub free_energies: Vec<f64>,
    pub density_of_states: Vec<f64>,
    pub volume: Vec<f64>,
    pub average_max_energies: Vec<f64>,
    pub max_energies: Vec<f64>,
    pub ge_de_vec: Vec<f64>,
}

impl SpitResults for PostProcessingResult {
    fn spit_ge_de(&self) {
        let mut file = std::fs::File::create("data/geDe.txt").unwrap();
        for i in 0..self.ge_de_vec.len() {
            file.write_all(format!("{}\n", self.ge_de_vec[i]).as_bytes())
                .unwrap();
        }
    }

    fn spit_max_energy(&self) {
        let mut file = std::fs::File::create("data/maxEnergy.txt").unwrap();
        for i in 0..self.max_energies.len() {
            file.write_all(format!("{}\n", self.max_energies[i]).as_bytes())
                .unwrap();
        }
    }
}

impl GraphResults for PostProcessingResult {
    fn graph_free_energies(&self) {
        let backend = plotting::create_plot_backend_png("graphs/freeEnergy_vs_maxEnergy.png");
        let x_range = (self.average_max_energies[self.average_max_energies.len() - 1] * 0.75)
            ..(self.average_max_energies[0] * 1.1);
        let y_range = (self.free_energies[0] * 0.75)
            ..(self.free_energies[self.free_energies.len() - 1] * 1.1);
        let drawing_area = backend.into_drawing_area();
        // println!("free energy y range: {:?}", y_range);
        plotting::plot_data(
            "Free Energy vs Max Energy",
            &drawing_area,
            x_range,
            y_range,
            self.average_max_energies.clone(),
            self.free_energies.clone(),
            plotting::Scale::LinearXLogY,
        );
    }

    fn graph_density_of_states(&self) {
        let backend = plotting::create_plot_backend_png("graphs/densityOfStates_vs_maxEnergy.png");
        let x_range = (self.average_max_energies[self.average_max_energies.len() - 1] * 0.75)
            ..(self.average_max_energies[0] * 1.1);
        let y_range = (self.density_of_states[self.density_of_states.len() - 1] * 0.75)
            ..(self.density_of_states[0] * 1.1);
        let drawing_area = backend.into_drawing_area();
        plotting::plot_data(
            "Density of States vs Max Energy",
            &drawing_area,
            x_range,
            y_range,
            self.average_max_energies.clone(), /* .into_iter().rev().collect() */
            self.density_of_states.clone(),    /* .into_iter().rev().collect() */
            plotting::Scale::LinearLinear,
        );
    }

    fn graph_volume(&self) {
        let backend = plotting::create_plot_backend_png("graphs/volume_vs_maxEnergy.png");
        let x_range =
            (self.max_energies[self.max_energies.len() - 1] * 0.75)..(self.max_energies[0] * 1.1);
        let y_range = (self.volume[self.volume.len() - 1] * 0.75)..(self.volume[0] * 1.1);
        let drawing_area = backend.into_drawing_area();
        plotting::plot_data(
            "Volume vs Max Energy",
            &drawing_area,
            x_range,
            y_range,
            self.max_energies.clone(),
            self.volume.clone(),
            plotting::Scale::LogLog,
        );
    }

    fn graph_ge_de(&self) {
        let backend = plotting::create_plot_backend_png("graphs/geDe_vs_maxEnergy.png");
        let x_range =
            (self.max_energies[self.max_energies.len() - 1] * 0.75)..(self.max_energies[0] * 1.1);
        let y_range = (self.ge_de_vec[self.ge_de_vec.len() - 1] * 0.75)..(self.ge_de_vec[0] * 1.1);
        let drawing_area = backend.into_drawing_area();
        plotting::plot_data(
            "Ge*De vs Max Energy",
            &drawing_area,
            x_range,
            y_range,
            self.max_energies.clone(),
            self.ge_de_vec.clone(),
            plotting::Scale::LinearLinear,
        );
    }
}

//# DOF = 3k (k = number of particles)
//beta = 1/t
//

fn boltzmann_weight(energy: f64, z: f64, beta: f64) -> f64 {
    // println!("numerator bw: {}", (-1.0 * beta * energy).exp());
    return (-1.0 * beta * energy).exp() / z;
}

/*
 * z = canonical partition function
 * beta = beta function
 * ge_de vec = vector of g(E)*dE values
 * max_energies = vector of max energies at iteration i
 * k = number of particles
*/

pub fn calculate_specific_heat(
    z: f64,
    beta: f64,
    ge_de_vec: Vec<f64>,
    i: i32,
    max_energies: Vec<f64>,
    k: usize,
    d: usize,
) -> f64 {
    let mut mean_of_square = 0.0;
    let mut mean = 0.0;

    for i in 0..i as usize {
        let element = ge_de_vec[i] * boltzmann_weight(max_energies[i], beta, z);
        println!(
            "ge_de_vec[i]: {}\nboltzmann weight: {}",
            ge_de_vec[i],
            boltzmann_weight(max_energies[i], beta, z)
        );
        mean += element;
        mean_of_square += element * element;
    }

    let ndof = (d * k) as f64;
    println!(
        "ndof: {}\nbeta: {}\nm_o_s: {}\nm: {}",
        ndof, beta, mean_of_square, mean
    );
    return ndof + ((beta * beta) * (mean_of_square - (mean * mean)));
}

pub fn beta_partition_calculation(ns_result: &ns_algo::NSResult, temperature: f64) -> (f64, f64) {
    //Beta funciton used in free energy
    let beta = 1.0 / temperature;

    //Canonical partition function
    let mut z = 0.0;

    for i in 0..(ns_result.iterations) as usize {
        z += (-1.0 * beta * ns_result.max_energies[i]).exp();
    }

    return (beta, z);
}

pub fn post_process(ns_result: ns_algo::NSResult) -> PostProcessingResult {
    //Find the density of states and the free energy
    let mut density_of_states: Vec<f64> = Vec::new();
    let mut free_energies: Vec<f64> = Vec::new();

    let mut average_max_energies: Vec<f64> = Vec::new();

    let mut volume: Vec<f64> = Vec::new();

    let mut ge_de_vec: Vec<f64> = Vec::new();

    for i in 0..(ns_result.iterations) as usize {
        let k_f = ns_result.k as f64;

        let ge_de = (1.0 / (k_f + 1.0)) * (k_f / (k_f + 1.0)).powf(i as f64);
        ge_de_vec.push(ge_de);

        //Find the moving average of the max energies over each iteration (for plotting against free
        //energy)
        if i != ns_result.max_energies.len() - 1 {
            average_max_energies
                .push((ns_result.max_energies[i] + ns_result.max_energies[i + 1]) / 2.0);
            density_of_states
                .push(ge_de / (ns_result.max_energies[i] - ns_result.max_energies[i + 1]));

            //TODO: account for the beta function in the free energy
            let free_energy = -1.0 * density_of_states[i].ln();
            free_energies.push(free_energy);
        }
    }

    //Arbitrary RANGE of temperatures
    let temperature_start = 0.0;

    let temperature_end = 100.0;

    //Determines the # of steps
    let temperature_scale = 1.0;

    //Betas and partition functions for different temperatures
    let mut betas: Vec<f64> = Vec::new();
    let mut zs: Vec<f64> = Vec::new();

    for i in (temperature_start * temperature_scale) as usize
        ..(temperature_end * temperature_scale) as usize
    {
        let t = i as f64 / temperature_scale;

        let (beta, z) = beta_partition_calculation(&ns_result, t);
        betas.push(beta);
        zs.push(z);
    }

    for i in 0..betas.len() {
        println!("z at iteration {}: {}", i, zs[i]);

        println!(
            "cv: {}",
            calculate_specific_heat(
                zs[i],
                betas[i],
                ge_de_vec.clone(),
                ns_result.iterations,
                ns_result.max_energies.clone(),
                ns_result.k,
                ns_result.dimensions,
            )
        );
    }

    let mut ge_de_sum: f64 = 0.0;
    //The volume array is now populated in order from min to max energy instead of iteration order
    for i in 0..(ns_result.iterations) as usize {
        //Take cumulative sum of gede in reverse
        volume.push(ge_de_sum);
        if ge_de_sum > 1.0 {
            println!("v: {}", ge_de_sum);
        }
        ge_de_sum += ge_de_vec[ge_de_vec.len() - 1 - i];
    }
    volume.reverse();

    return PostProcessingResult {
        free_energies: free_energies,
        density_of_states: density_of_states,
        volume: volume,
        average_max_energies: average_max_energies,
        max_energies: ns_result.max_energies,
        ge_de_vec: ge_de_vec,
    };
}
