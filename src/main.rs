use crate::plots::{generate_plots, PLOT_SAMPLING};

pub const MAX_MONEY: i64 = 21_000_000;
const BLOCKS_PER_YEAR: u32 = 420_768;
const ZATOSHIS_PER_ZEC: i64 = 100_000_000;
// predicted ZEC supply at the next halving (block 2726400)
const INITIAL_SUPPLY: i64 = 1_574_963_454_129_680;
const INITIAL_SUBSIDIES: i64 = MAX_MONEY * ZATOSHIS_PER_ZEC - INITIAL_SUPPLY;

const BLOCK_SUBSIDY_NUMERATOR: i64 = 4126;
const BLOCK_SUBSIDY_DENOMINATOR: i64 = 10_000_000_000;

const POST_BLOSSOM_HALVING_INTERVAL: u32 = 1_680_000;
// block subsidy starting at the next halving (block 2726400)
const INITIAL_LEGACY_BLOCK_SUBSIDY: i64 = 50 * ZATOSHIS_PER_ZEC / 32;

mod plots;

fn main() {
    let (block, balance_points, zsf_reward_points, legacy_reward_points, halvings) = simulate();

    print_halving(halvings);

    let years = block as f64 / BLOCKS_PER_YEAR as f64;

    generate_plots(
        balance_points,
        zsf_reward_points,
        legacy_reward_points,
        years,
    );
}

fn simulate() -> (
    u32,
    Vec<(f64, f64)>,
    Vec<(f64, f64)>,
    Vec<(f64, f64)>,
    Vec<i64>,
) {
    let mut available_subsidies: i64 = INITIAL_SUBSIDIES;
    let mut block: u32 = 0;

    let mut balance_points: Vec<(f64, f64)> = Vec::new();
    let mut zsf_reward_points: Vec<(f64, f64)> = Vec::new();
    let mut legacy_reward_points: Vec<(f64, f64)> = Vec::new();
    let mut halvings: Vec<i64> = Vec::new();
    let mut halving_subsidies: i64 = 0;

    while available_subsidies > 0 {
        let block_subsidy = (available_subsidies * BLOCK_SUBSIDY_NUMERATOR
            + (BLOCK_SUBSIDY_DENOMINATOR - 1))
            / BLOCK_SUBSIDY_DENOMINATOR;
        available_subsidies -= block_subsidy;
        halving_subsidies += block_subsidy;

        if available_subsidies == 0 {
            println!(
                "Last block {} (~{:.2} years): Block subsidy: {} (~{} ZEC), ZSF balance: {} (~{} ZEC)",
                block,                                  // current block
                block as f64 / BLOCKS_PER_YEAR as f64,  // ~ current year
                block_subsidy,                          // block subsidy in zatoshis
                block_subsidy / ZATOSHIS_PER_ZEC,       // block subsidy in ZEC
                available_subsidies,                    // available subsidies in zatoshis
                available_subsidies / ZATOSHIS_PER_ZEC  // available subsidies in ZEC
            );
        }

        if block % PLOT_SAMPLING == 0 {
            let block_float = block as f64 / BLOCKS_PER_YEAR as f64;
            balance_points.push((
                block_float,
                available_subsidies as f64 / ZATOSHIS_PER_ZEC as f64,
            ));

            zsf_reward_points.push((block_float, block_subsidy as f64 / ZATOSHIS_PER_ZEC as f64));

            legacy_reward_points.push((
                block_float,
                ((INITIAL_LEGACY_BLOCK_SUBSIDY / 2_i64.pow(block / POST_BLOSSOM_HALVING_INTERVAL))
                    as f64)
                    / ZATOSHIS_PER_ZEC as f64,
            ));
        }

        if block > 0 && block % POST_BLOSSOM_HALVING_INTERVAL == 0 {
            halvings.push(halving_subsidies);
            halving_subsidies = 0;
        }

        block += 1;
    }
    (
        block,
        balance_points,
        zsf_reward_points,
        legacy_reward_points,
        halvings,
    )
}

fn print_halving(halvings: Vec<i64>) {
    println!("#############################################");
    println!("Halvings:");

    for (i, zsf_halving_subsidies) in halvings.iter().enumerate() {
        let legacy_subsidy_per_block = INITIAL_LEGACY_BLOCK_SUBSIDY / 2_i64.pow(i as u32);
        let legacy_halving_subsidies = legacy_subsidies_for_halving(i as u32);
        let difference = zsf_halving_subsidies - legacy_halving_subsidies;
        let ratio = *zsf_halving_subsidies as f64 / legacy_halving_subsidies as f64;

        println!(
            "Halving {:>2} at block {:>8}:\n  ZSF subsidies:    {:>15} (~{:>7} ZEC)\n  legacy subsidies: {:>15} (~{:>7} ZEC) ({:.8} ZEC per block)\n  difference:       {:>15} (~{:>7} ZEC), ZSF/legacy: {:.4}",
            i + 1,
            (i + 1) as u32 * POST_BLOSSOM_HALVING_INTERVAL,
            zsf_halving_subsidies,
            zsf_halving_subsidies / ZATOSHIS_PER_ZEC,
            legacy_halving_subsidies,
            legacy_halving_subsidies / ZATOSHIS_PER_ZEC,
            legacy_subsidy_per_block as f64 / ZATOSHIS_PER_ZEC as f64,
            difference,
            difference / ZATOSHIS_PER_ZEC,
            ratio
        );
    }

    println!("#############################################");
}

fn legacy_subsidies_for_halving(halving: u32) -> i64 {
    let subsidy_for_halving = INITIAL_LEGACY_BLOCK_SUBSIDY / 2_i64.pow(halving);
    POST_BLOSSOM_HALVING_INTERVAL as i64 * subsidy_for_halving
}
