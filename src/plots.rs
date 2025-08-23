use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, PathElement},
    series::LineSeries,
    style::*,
};

use crate::{INITIAL_HALVING, MAX_MONEY};

const BALANCE_PLOT_PATH: &str = "plots/nsm_balance.png";
const BLOCK_SUBSIDY_PLOT_PATH: &str = "plots/nsm_block_subsidy.png";
const PLOT_ZOOM: usize = 5;
pub const PLOT_SAMPLING: u32 = 1000;

pub fn generate_plots(
    balance_points: Vec<(f64, f64)>,
    nsm_reward_points: Vec<(f64, f64)>,
    no_nsm_reward_points: Vec<(f64, f64)>,
    years: f64,
) {
    balance_plot(
        &balance_points[0..(balance_points.len() / PLOT_ZOOM)],
        years / PLOT_ZOOM as f64,
    );

    block_subsidy_plot(
        &nsm_reward_points[0..(balance_points.len() / PLOT_ZOOM)],
        &no_nsm_reward_points[0..(balance_points.len() / PLOT_ZOOM)],
        years / PLOT_ZOOM as f64,
    );
}

fn balance_plot(points: &[(f64, f64)], years: f64) {
    let root = BitMapBackend::new(BALANCE_PLOT_PATH, (1024, 768)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("NSM balance", ("sans-serif", 50).into_font())
        .margin(50)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..years, 0f64..(MAX_MONEY as f64))
        .unwrap();

    chart
        .configure_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("NSM balance in ZEC")
        .x_desc(format!("Years from {INITIAL_HALVING}nd halving"))
        .axis_desc_style(("sans-serif", 15))
        .y_label_formatter(&|x| format!("{}M", x / 1_000_000_f64))
        .draw()
        .unwrap();

    let series = LineSeries::new(points.to_owned(), &RED);

    chart.draw_series(series).unwrap();

    root.present().unwrap();

    println!("Plot saved to {}", BALANCE_PLOT_PATH);
}

fn block_subsidy_plot(
    nsm_reward_points: &[(f64, f64)],
    no_nsm_reward_points: &[(f64, f64)],
    years: f64,
) {
    let root = BitMapBackend::new(BLOCK_SUBSIDY_PLOT_PATH, (1024, 768)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Block subsidies - NSM vs current",
            ("sans-serif", 50).into_font(),
        )
        .margin(50)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..years, 0f64..5f64)
        .unwrap();

    chart
        .configure_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Block subsidy in ZEC")
        .x_desc(format!("Years from {INITIAL_HALVING}nd halving"))
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .unwrap();

    let nsm_series = LineSeries::new(nsm_reward_points.to_owned(), &RED);
    let no_nsm_series = LineSeries::new(no_nsm_reward_points.to_owned(), &BLUE);

    chart
        .draw_series(nsm_series)
        .unwrap()
        .label("NSM Smooth Issuance")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(no_nsm_series)
        .unwrap()
        .label("Current issuance")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()
        .unwrap();
    root.present().unwrap();

    println!("Plot saved to {}", BLOCK_SUBSIDY_PLOT_PATH);
}
