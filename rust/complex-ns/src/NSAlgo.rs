use ndarray::Array;
use rand;

pub type Energy_Function<d> = fn(Array<f64, d>, Array<f64, d>) -> Array<f64, ndarray::Array1<f64>>;

pub struct NS_Config<d> {
    energy_function: Energy_Function<d>,
    initial_state: Array<f64, d>,
    iterations: i32,
    //TODO: implement a struct containing various debug options (debug conf)
    debug: bool,
    rng: rand::rngs::ThreadRng,
}

pub struct Min_Energy<d> {
    pub energy: f64,
    pub state: Array<f64, d>,
    pub replica_idx: usize,
}

pub struct NS_Result<d> {
    max_energies: Array<f64, ndarray::Array1<f64>>,
    min_energy: Min_Energy<d>,
}

// pub fn NS_Algo<d>(config: NS_Config<d>) -> NS_Result<d> {
//
// }
