pub fn fourth_degree_energy_func(x: f64) -> f64 {
    return 2.0 * x.powi(4) - (3.0 * x.powi(3)) - (7.0 * x.powi(2)) + (2.0 * x) + 1.0;
}


pub fn complex_energy_func(x: f64) -> f64 {
    return 9.7 * ((x/7.0).powi(2)).sin() -  (4.0 * ((x/8.0).cos())).powi(2);
}

pub fn square(x: f64) -> f64 {
    return x.powi(2);
}
