use rand::{distributions::Uniform, prelude::Distribution};
use plotpy::{Curve, Plot};

pub fn random_point(state: &Vec<Vec<f64>>, rng: &mut rand::rngs::ThreadRng) -> Vec<f64> {
    let state_idx_gen = Uniform::from(0..state.len());
    let state_idx = state_idx_gen.sample(rng);
    return state[state_idx].clone();
}


pub fn states_populate(dimensions: usize, count: usize, range: std::ops::Range<f64>, rng: &mut rand::rngs::ThreadRng) -> Vec<Vec<f64>> {
    let mut states: Vec<Vec<f64>> = Vec::new();
    let dist = Uniform::from(range);

    for _ in 0..count {
        let mut sample: Vec<f64> = Vec::new();
        for _ in 0..dimensions {
            sample.push(dist.sample(rng));
        }
        states.push(sample);
    }
    
    println!("Initial States Have Been Uniformly Populated");

    return states;
}

pub fn plot_data(
    plot: &mut Plot,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    title: &str,
    x_label: &str,
    y_label: &str,
) -> Curve {
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

    return curve;
}

pub fn update_plot(
    plot: &mut Plot,
    curve: &mut Curve,
    x_value: f64,
    y_value: f64,
) {
    // draw curve
    curve.points_add(&x_value, &y_value);

    // save figure
    let _ = plot.save_and_show("./plot.svg");
}
