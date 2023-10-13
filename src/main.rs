use crate::plots::{generate_plots, PLOT_SAMPLING};

pub const MAX_MONEY: i64 = 21_000_000;
const BLOCKS_PER_YEAR: u32 = 420_768;
const ZATOSHIS_PER_ZEC: i64 = 100_000_000;
// predicted ZEC supply at the next halving (block 2726400)
const INITIAL_SUPPLY: i64 = 1_574_963_454_129_680;
const INITIAL_SUBSIDIES: i64 = MAX_MONEY * ZATOSHIS_PER_ZEC - INITIAL_SUPPLY;

const BLOCK_SUBSIDY_NUMERATOR: i64 = 4126;
const BLOCK_SUBSIDY_DENOMINATOR: i64 = 10_000_000_000;

mod plots;

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

        if block % PLOT_SAMPLING == 0 {
            points.push((
                block as f64 / BLOCKS_PER_YEAR as f64,
                available_subsidies as f64 / ZATOSHIS_PER_ZEC as f64,
            ));
        }

        block += 1;
    }

    println!("#############################################");
    println!("Final block: {}", block - 1);
    println!("#############################################");

    let years = block as f64 / BLOCKS_PER_YEAR as f64;

    generate_plots(points, years);
}
