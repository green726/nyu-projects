pub(crate) fn fourth_degree_energy_func(x: f64) -> f64 {
    return 2.0 * x.powi(4) - (3.0 * x.powi(3)) - (7.0 * x.powi(2)) + (2.0 * x) + 1.0;
}
