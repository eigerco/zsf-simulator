use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, PathElement},
    series::LineSeries,
    style::*,
};

use crate::MAX_MONEY;

const BALANCE_PLOT_PATH: &str = "plots/zsf_balance.png";
const BLOCK_SUBSIDY_PLOT_PATH: &str = "plots/zsf_block_subsidy.png";
const PLOT_ZOOM: usize = 5;
pub const PLOT_SAMPLING: u32 = 1000;

pub fn generate_plots(
    balance_points: Vec<(f64, f64)>,
    zsf_reward_points: Vec<(f64, f64)>,
    legacy_reward_points: Vec<(f64, f64)>,
    years: f64,
) {
    balance_plot(
        &balance_points[0..(balance_points.len() / PLOT_ZOOM)],
        years / PLOT_ZOOM as f64,
    );

    block_subsidy_plot(
        &zsf_reward_points[0..(balance_points.len() / PLOT_ZOOM)],
        &legacy_reward_points[0..(balance_points.len() / PLOT_ZOOM)],
        years / PLOT_ZOOM as f64,
    );
}

fn balance_plot(points: &[(f64, f64)], years: f64) {
    let root = BitMapBackend::new(BALANCE_PLOT_PATH, (1024, 768)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("ZSF balance", ("sans-serif", 50).into_font())
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

    println!("Plot saved to {}", BALANCE_PLOT_PATH);
}

fn block_subsidy_plot(
    zsf_reward_points: &[(f64, f64)],
    legacy_reward_points: &[(f64, f64)],
    years: f64,
) {
    let root = BitMapBackend::new(BLOCK_SUBSIDY_PLOT_PATH, (1024, 768)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Block subsidies - ZSF vs current",
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
        .x_desc("Years from activation")
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .unwrap();

    let zsf_series = LineSeries::new(zsf_reward_points.to_owned(), &RED);
    let legacy_series = LineSeries::new(legacy_reward_points.to_owned(), &BLUE);

    chart
        .draw_series(zsf_series)
        .unwrap()
        .label("ZSF Smooth Issuance")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(legacy_series)
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
