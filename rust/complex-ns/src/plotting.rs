use plotters::prelude::*;


pub fn create_plot_backend_gif<'a>(filename_and_path: &str) -> BitMapBackend<'a> {
    let backend: BitMapBackend<'_> = BitMapBackend::gif(filename_and_path, (800, 600), 1).unwrap();
    return backend;
}

pub fn create_plot_backend_png<'a>(filename_and_path: &'a str) -> BitMapBackend<'a> {
    let backend: BitMapBackend<'_> = BitMapBackend::new(filename_and_path, (800, 600));
    return backend;
}

pub fn plot_data(
    name: &str,
    root_drawing_area: &DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    x_range: std::ops::Range<f64>,
    y_range: std::ops::Range<f64>,
    x: Vec<f64>,
    y: Vec<f64>,
) {
    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption(name, ("Arial", 30))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_range, y_range)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new(
        x.iter()
            .zip(y.iter())
            .map(|(x, y)| (*x as f64, *y as f64)),
        &RED,
    ).point_size(2)).unwrap();

    root_drawing_area.present().unwrap();
}
