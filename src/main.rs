use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea},
    series::LineSeries,
    style::*,
};

const BLOCKS_PER_YEAR: u32 = 420_768;
const ZATOSHIS_PER_ZEC: i64 = 100_000_000;
// predicted ZEC supply at the next halving (block 2726400)
const INITIAL_SUPPLY: i64 = 1_574_963_454_129_680;
const MAX_MONEY: i64 = 21_000_000;
const INITIAL_SUBSIDIES: i64 = MAX_MONEY * ZATOSHIS_PER_ZEC - INITIAL_SUPPLY;

const BLOCK_SUBSIDY_NUMERATOR: i64 = 4126;
const BLOCK_SUBSIDY_DENOMINATOR: i64 = 10_000_000_000;

const FULL_PLOT_PATH: &str = "plots/zsf_issuance_plot_full.png";
const SHORT_PLOT_PATH: &str = "plots/zsf_issuance_plot_short.png";
const SHORT_PLOT_DIVISOR: usize = 5;
const PLOT_EVERY: u32 = 1000;

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

fn main() {
    let mut available_subsidies: i64 = INITIAL_SUBSIDIES;
    let mut block: u32 = 0;

    let mut points: Vec<(f64, f64)> = Vec::new();

    while available_subsidies > 0 {
        let block_subsidy = (available_subsidies * BLOCK_SUBSIDY_NUMERATOR
            + (BLOCK_SUBSIDY_DENOMINATOR - 1))
            / BLOCK_SUBSIDY_DENOMINATOR;
        available_subsidies -= block_subsidy;

        println!(
            "Block {} (~{:.2} years): Subsidy: {} (~{} ZEC), ZSF: {} (~{} ZEC)",
            block,                                  // current block
            block as f64 / BLOCKS_PER_YEAR as f64,  // ~ current year
            block_subsidy,                          // block subsidy in zatoshis
            block_subsidy / ZATOSHIS_PER_ZEC,       // block subsidy in ZEC
            available_subsidies,                    // available subsidies in zatoshis
            available_subsidies / ZATOSHIS_PER_ZEC  // available subsidies in ZEC
        );

        if block % PLOT_EVERY == 0 {
            points.push((
                block as f64 / BLOCKS_PER_YEAR as f64,
                available_subsidies as f64 / ZATOSHIS_PER_ZEC as f64,
            ));
        }

        block += 1;
    }

    println!("#############################################");
    println!("Final block: {}", block);
    println!("#############################################");

    let years = block as f64 / BLOCKS_PER_YEAR as f64;

    // plot the entire range
    plot(&points, years, FULL_PLOT_PATH);

    // plot the initial part of the graph
    plot(
        &points[0..(points.len() / SHORT_PLOT_DIVISOR)],
        years / SHORT_PLOT_DIVISOR as f64,
        SHORT_PLOT_PATH,
    );
}
