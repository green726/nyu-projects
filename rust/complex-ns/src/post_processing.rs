use crate::{ns_algo, plotting};
use plotters::prelude::*;

pub trait GraphResults {
    fn graph_free_energies(&self);
    fn graph_density_of_states(&self);
    fn graph_volume(&self);
}

pub struct PostProcessingResult {
    pub free_energies: Vec<f64>,
    pub density_of_states: Vec<f64>,
    pub volume: Vec<f64>,
    pub average_max_energies: Vec<f64>,
    pub max_energies: Vec<f64>,
}

impl GraphResults for PostProcessingResult {
    fn graph_free_energies(&self) {
        let backend = plotting::create_plot_backend_png("graphs/freeEnergy_vs_maxEnergy.png");
        let x_range = (self.average_max_energies[0] * 0.75) as i32
            ..(self.average_max_energies[self.average_max_energies.len() - 1] * 1.1) as i32;
        let y_range = (self.free_energies[0] * 0.75) as i32
            ..(self.free_energies[self.free_energies.len() - 1] * 1.1) as i32;
        let drawing_area = backend.into_drawing_area();
        plotting::plot_data_floatx(
            "Free Energy vs Max Energy",
            &drawing_area,
            x_range,
            y_range,
            self.average_max_energies.clone(),
            self.free_energies.clone(),
        );
    }

    fn graph_density_of_states(&self) {
        let backend = plotting::create_plot_backend_png("graphs/densityOfStates_vs_maxEnergy.png");
        let x_range = (self.average_max_energies[0] * 0.75) as i32
            ..(self.average_max_energies[self.average_max_energies.len() - 1] * 1.1) as i32;
        let y_range = (self.density_of_states[0] * 0.75) as i32
            ..(self.density_of_states[self.density_of_states.len() - 1] * 1.1) as i32;
        let drawing_area = backend.into_drawing_area();
        plotting::plot_data_floatx(
            "Density of States vs Max Energy",
            &drawing_area,
            x_range,
            y_range,
            self.average_max_energies.clone(),
            self.density_of_states.clone(),
        );
    }

    fn graph_volume(&self) {
        let backend = plotting::create_plot_backend_png("graphs/volume_vs_maxEnergy.png");
        let x_range =
            (self.max_energies[0] * 0.75) as i32..(self.max_energies[self.max_energies.len() - 1] * 1.1) as i32;
        let y_range = (self.volume[0] * 0.75) as i32..(self.volume[self.volume.len() - 1] * 1.1) as i32;
        let drawing_area = backend.into_drawing_area();
        plotting::plot_data_floatx(
            "Volume vs Max Energy",
            &drawing_area,
            x_range,
            y_range,
            self.max_energies.clone(),
            self.volume.clone(),
        );
    }
}

pub fn post_process(ns_result: ns_algo::NSResult) -> PostProcessingResult {
    //Find the density of states and the free energy

    let mut density_of_states: Vec<f64> = Vec::new();
    let mut free_energies: Vec<f64> = Vec::new();
    for i in 0..ns_result.max_energies.len() {
        let k_f = ns_result.k as f64;
        let density_of_state = (1.0 / (k_f + 1.0) * (k_f / (k_f + 1.0)).powf(i as f64));
        density_of_states.push(density_of_state);

        //TODO: account for the beta function in the free energy
        let free_energy = -1.0 * density_of_state.ln();
        free_energies.push(free_energy);
    }

    //Find the moving average of the max energies over each iteration (for plotting against free
    //energy)
    let mut average_max_energies: Vec<f64> = Vec::new();

    for i in 0..ns_result.max_energies.len() {
        if i != ns_result.max_energies.len() - 1 {
            average_max_energies
                .push((ns_result.max_energies[i] + ns_result.max_energies[i + 1]) / 2.0);
        } else {
            average_max_energies.push(ns_result.max_energies[i]);
        }
    }

    //Find the state counts (volume)
    let mut volume: Vec<f64> = Vec::new();
    for i in 0..ns_result.max_energies.len() {
        volume
            .push(density_of_states[i] * (ns_result.max_energies[i] - ns_result.min_energy.energy));
    }

    return PostProcessingResult {
        free_energies: free_energies,
        density_of_states: density_of_states,
        volume: volume,
        average_max_energies: average_max_energies,
        max_energies: ns_result.max_energies,
    };
}
