use rand;

pub type EnergyFunction = fn(Vec<Vec<f64>>) -> f64;

pub struct NsConfig {
    energy_function: EnergyFunction,
    initial_state: Vec<Vec<f64>>,
    iterations: i32,
    //TODO: implement a struct containing various debug options (debug conf)
    debug: bool,
    rng: rand::rngs::ThreadRng,
}

pub struct MinEnergy {
    pub energy: f64,
    pub state: Vec<Vec<f64>>,
    pub replica_idx: usize,
}

pub struct NsResult {
    max_energies: Vec<f64>,
    min_energy: MinEnergy,
}

// pub fn NS_Algo<d>(config: NS_Config<d>) -> NS_Result<d> {
//
// }
