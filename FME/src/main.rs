mod utils;
mod types;

use std::env;
use std::fs;
use types::{Event, Cashflow, Asset, Portfolio, Debt, MonthlyOutput};
use num_format::{Locale, ToFormattedString};

const YEARS_TO_RUN: u32 = 20 * 12;
// 8% divided monthly
const MONTHLY_DEBT_INTEREST_RATE: f32 = 0.08 / 12.0;
// 10% divided monthly
const MONTHLY_ASSET_APPRECIATION: f32 = 0.10 / 12.0;

fn main() {
    let args: Vec<String> = env::args().collect();
    let folder = &args[1];
    let events: Vec<Event> = utils::read_file(format!("./inputs/{}/events.csv", folder));
    let mut cashflow: Vec<Cashflow> = utils::read_file(format!("./inputs/{}/cashflow.csv", folder));
    let mut portfolio: Vec<Asset> = utils::read_file(format!("./inputs/{}/portfolio.csv", folder));
    let mut debt: Vec<Debt> = Vec::new();

    fs::create_dir_all(format!("../outputs/{}", &folder)).unwrap();
    let monthly_portfolio_sum_file = fs::File::create(format!("../outputs/{}/monthly_portfolio_sum.csv", &folder)).unwrap();
    let mut wtr = csv::Writer::from_writer(monthly_portfolio_sum_file);
    // Engine Loop
    for month in 0..YEARS_TO_RUN {
        // Handle Events
        let current_events: Vec<&Event> = events.iter().filter(|event| event.month == month).collect();
        if !current_events.is_empty() {
            for current_event in current_events {
                // Cashflow
                cashflow.push( Cashflow { month: month, name: current_event.name.clone(), amount: current_event.cashflow });
                // Assets
                if current_event.asset {
                    portfolio.push( Asset { month: month, name: current_event.name.clone(), value: current_event.debt });
                }
                // Debt
                if current_event.debt != 0.0 {
                    debt.push(Debt { month: month, amount: current_event.debt });
                    let interest_payment = current_event.debt * MONTHLY_DEBT_INTEREST_RATE;
                    // Add interest payments (negative) to cashflow
                    cashflow.push( Cashflow { month: month, name: current_event.name.clone(), amount: -interest_payment });
                }
            }
        }
        
        // Add monthly gains to portfolio
        let mut total_portfolio_value: f32 = 0.0;
        for asset in &portfolio {
            total_portfolio_value += asset.value
        }
        let monthly_gain = total_portfolio_value * MONTHLY_ASSET_APPRECIATION;
        portfolio.push( Asset { month: month, name: format!("Appreciation {}", month), value: monthly_gain });

        // Divert excess cashflow to portfolio (securities)
        let mut monthly_excess_cashflow:f32 = 0.0;
        for val in &cashflow {
            monthly_excess_cashflow += val.amount;
        }

        portfolio.push( Asset { month: month, name: format!("Excess Cashflow {}", month), value: monthly_excess_cashflow });

        let portfolio_total = utils::sum_portfolio(&portfolio) as i64;
        let debt_total = utils::sum_debt(&debt) as i64;
        let output = MonthlyOutput {
            month: month,
            portfolio_sum: portfolio_total,
            debt_sum: debt_total,
            cashflow_sum: monthly_excess_cashflow as i64
        };

        wtr.serialize(output).unwrap();
    }
    wtr.flush().unwrap();
}
