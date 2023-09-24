use plotters::prelude::*;

pub fn create_plot_backend<'a>() -> BitMapBackend<'a> {
    let backend: BitMapBackend<'_> = BitMapBackend::gif("images/animated.gif", (600, 400), 3).unwrap();
    return backend;
}

pub fn plot_data_intx(
    name: &str,
    root_drawing_area: &DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    x_range: std::ops::Range<i32>,
    y_range: std::ops::Range<i32>,
    x: Vec<i32>,
    y: Vec<f64>,
) {
    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption(name, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_range, y_range)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new(
        x.iter()
            .zip(y.iter())
            .map(|(x, y)| (*x as i32, *y as i32)),
        &RED,
    )).unwrap();

    root_drawing_area.present().unwrap();
}

pub fn plot_data_floatx(
    name: &str,
    root_drawing_area: &DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>,
    x_range: std::ops::Range<i32>,
    y_range: std::ops::Range<i32>,
    x: Vec<f64>,
    y: Vec<f64>,
) {
    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption(name, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_range, y_range)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new(
        x.iter()
            .zip(y.iter())
            .map(|(x, y)| (*x as i32, *y as i32)),
        &RED,
    )).unwrap();

    root_drawing_area.present().unwrap();
}
