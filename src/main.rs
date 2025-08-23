use crate::plots::{generate_plots, PLOT_SAMPLING};

mod plots;

type Height = i64; // or height interval
type Zats = i64;

pub const MAX_MONEY: Zats = 21_000_000;
const BLOCKS_PER_YEAR: i64 = 420_768;
const ZATOSHIS_PER_ZEC: Zats = 1_0000_0000;
const INITIAL_HALVING: usize = 2;
const INITIAL_HALVING_HEIGHT: Height = 2726400;
// Predicted ZEC supply at INITIAL_HALVING. TODO: check this.
const INITIAL_SUPPLY: Zats = 1_574_963_454_129_680;
const INITIAL_SUBSIDIES: Zats = MAX_MONEY * ZATOSHIS_PER_ZEC - INITIAL_SUPPLY;

const NSM_BLOCK_SUBSIDY_NUMERATOR: i64 = 4126;
const NSM_BLOCK_SUBSIDY_DENOMINATOR: i64 = 10_000_000_000;

const POST_BLOSSOM_HALVING_INTERVAL: Height = 1_680_000;
// Block subsidy at INITIAL_HALVING_HEIGHT.
const INITIAL_NO_NSM_BLOCK_SUBSIDY: Zats = 12_5000_0000 / 8;

fn to_zec(zatoshis: Zats) -> f64 {
    zatoshis as f64 / ZATOSHIS_PER_ZEC as f64
}

fn years_after_initial(height: Height) -> f64 {
    (height - INITIAL_HALVING_HEIGHT) as f64 / BLOCKS_PER_YEAR as f64
}

fn no_nsm_block_subsidy_for_height(height: Height) -> Zats {
    assert!(height >= INITIAL_HALVING_HEIGHT);
    let halving_after_initial = (height - INITIAL_HALVING_HEIGHT) / POST_BLOSSOM_HALVING_INTERVAL;
    INITIAL_NO_NSM_BLOCK_SUBSIDY >> std::cmp::min(halving_after_initial, 63)
}

struct Stats {
    pub index: i64,
    pub end_height: i64,
    pub nsm_subsidies: i64,
    pub no_nsm_subsidies: i64,
}

impl Stats {
    fn summary(self: &Stats) -> String {
        let difference = self.nsm_subsidies - self.no_nsm_subsidies;
        let ratio = self.nsm_subsidies as f64 / self.no_nsm_subsidies as f64;
        let end_year = self.index * 4;
        format!(
            "Years {:>3}..{:>3} at heights {:>8}..{:>8}:\n  \
               NSM subsidies:    {:>15} (~{:>12.3} ZEC, {:>12.8} ZEC per block)\n  \
               no-NSM subsidies: {:>15} (~{:>12.3} ZEC, {:>12.8} ZEC per block)\n  \
               difference:       {:>15} (~{:>12.3} ZEC),         NSM/no-NSM: {:.4}",
            end_year - 4,
            end_year,
            self.end_height - POST_BLOSSOM_HALVING_INTERVAL,
            self.end_height,
            self.nsm_subsidies,
            to_zec(self.nsm_subsidies),
            self.nsm_subsidies as f64
                / POST_BLOSSOM_HALVING_INTERVAL as f64
                / ZATOSHIS_PER_ZEC as f64,
            self.no_nsm_subsidies,
            to_zec(self.no_nsm_subsidies),
            self.no_nsm_subsidies as f64
                / POST_BLOSSOM_HALVING_INTERVAL as f64
                / ZATOSHIS_PER_ZEC as f64,
            difference,
            to_zec(difference),
            ratio
        )
    }
}

fn main() {
    let (end_height, balance_points, nsm_reward_points, no_nsm_reward_points, four_year_stats) =
        simulate();

    print_four_year_stats(&four_year_stats);

    generate_plots(
        balance_points,
        nsm_reward_points,
        no_nsm_reward_points,
        years_after_initial(end_height),
    );
}

#[allow(clippy::type_complexity)]
fn simulate() -> (
    Height,
    Vec<(f64, f64)>,
    Vec<(f64, f64)>,
    Vec<(f64, f64)>,
    Vec<Stats>,
) {
    let mut available_subsidies: Zats = INITIAL_SUBSIDIES;
    let mut height: Height = INITIAL_HALVING_HEIGHT;

    let mut balance_points: Vec<(f64, f64)> = Vec::new();
    let mut nsm_reward_points: Vec<(f64, f64)> = Vec::new();
    let mut no_nsm_reward_points: Vec<(f64, f64)> = Vec::new();
    let mut four_year_stats: Vec<Stats> = Vec::new();
    let mut four_year_nsm_subsidies: Zats = 0;
    let mut four_year_no_nsm_subsidies: Zats = 0;
    let mut nsm_deployment_height: Option<Height> = None;

    while available_subsidies > 0 {
        let years = years_after_initial(height);
        let nsm_block_subsidy = (available_subsidies * NSM_BLOCK_SUBSIDY_NUMERATOR
            + (NSM_BLOCK_SUBSIDY_DENOMINATOR - 1))
            / NSM_BLOCK_SUBSIDY_DENOMINATOR;

        let no_nsm_block_subsidy = no_nsm_block_subsidy_for_height(height);

        if nsm_deployment_height.is_none() && nsm_block_subsidy < no_nsm_block_subsidy {
            nsm_deployment_height = Some(height);
            println!("DEPLOYMENT_BLOCK_HEIGHT = {height} ({years:.2} years after {INITIAL_HALVING}nd halving)");
        }

        let block_subsidy = if nsm_deployment_height.is_some() {
            four_year_nsm_subsidies += nsm_block_subsidy;
            four_year_no_nsm_subsidies += no_nsm_block_subsidy;
            nsm_block_subsidy
        } else {
            no_nsm_block_subsidy
        };
        available_subsidies -= block_subsidy;

        if available_subsidies == 0 {
            println!(
                "Last block with non-zero subsidy is at height {} in ~{:.2} years after the {}nd halving.\n\
                 Final block subsidy: {} (~{} ZEC)\n\
                 Final NSM balance: {} (~{} ZEC)",
                height,                     // current height
                years,                      // ~ current year after initial halving
                INITIAL_HALVING,            // initial halving ordinal
                block_subsidy,              // block subsidy in zatoshis
                to_zec(block_subsidy),      // block subsidy in ZEC
                available_subsidies,        // available subsidies in zatoshis
                to_zec(available_subsidies) // available subsidies in ZEC
            );
        }

        if ((height - INITIAL_HALVING_HEIGHT) as u32) % PLOT_SAMPLING == 0 {
            balance_points.push((years, to_zec(available_subsidies)));
            nsm_reward_points.push((years, to_zec(block_subsidy)));
            no_nsm_reward_points.push((years, to_zec(no_nsm_block_subsidy)));
        }

        if let Some(h) = nsm_deployment_height {
            if height > h && (height - h) % POST_BLOSSOM_HALVING_INTERVAL == 0 {
                four_year_stats.push(Stats {
                    index: (height - h) / POST_BLOSSOM_HALVING_INTERVAL,
                    end_height: height,
                    nsm_subsidies: four_year_nsm_subsidies,
                    no_nsm_subsidies: four_year_no_nsm_subsidies,
                });
                four_year_nsm_subsidies = 0;
                four_year_no_nsm_subsidies = 0;
            }
        }

        height += 1;
    }
    (
        height,
        balance_points,
        nsm_reward_points,
        no_nsm_reward_points,
        four_year_stats,
    )
}

fn print_four_year_stats(four_year_stats: &Vec<Stats>) {
    println!("#############################################");
    println!("Four-year periods:");

    for stats in four_year_stats {
        println!("{}", stats.summary());
    }

    println!("#############################################");
}
