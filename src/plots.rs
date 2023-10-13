use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea},
    series::LineSeries,
    style::*,
};

use crate::MAX_MONEY;

const FULL_PLOT_PATH: &str = "plots/zsf_issuance_plot_full.png";
const SHORT_PLOT_PATH: &str = "plots/zsf_issuance_plot_short.png";
const SHORT_PLOT_DIVISOR: usize = 5;
pub const PLOT_SAMPLING: u32 = 1000;

pub fn generate_plots(points: Vec<(f64, f64)>, years: f64) {
    // plot the entire range
    plot(&points, years, FULL_PLOT_PATH);

    // plot the initial part of the graph
    plot(
        &points[0..(points.len() / SHORT_PLOT_DIVISOR)],
        years / SHORT_PLOT_DIVISOR as f64,
        SHORT_PLOT_PATH,
    );
}

fn plot(points: &[(f64, f64)], years: f64, path: &str) {
    let root = BitMapBackend::new(path, (1024, 768)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("ZSF issuance curve", ("sans-serif", 50).into_font())
        .margin(50)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..years, 0f64..(MAX_MONEY as f64))
        .unwrap();

    chart
        .configure_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("ZSF balance in ZEC")
        .x_desc("Years from activation")
        .axis_desc_style(("sans-serif", 15))
        .y_label_formatter(&|x| format!("{}M", x / 1_000_000_f64))
        .draw()
        .unwrap();

    let series = LineSeries::new(points.to_owned(), &RED);

    chart.draw_series(series).unwrap();

    root.present().unwrap();

    println!("Plot saved to {}", path);
}
