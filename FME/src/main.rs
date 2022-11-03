mod utils;
mod engine;
mod types;

use std::{env, io};
use types::{Event, Cashflow, Asset, Portfolio, Debt, MonthlyOutput};



fn main() {
    let args: Vec<String> = env::args().collect();
    let folder = &args[1];
    let yearly_debt_interest_rate: f32 = args.get(2).unwrap_or(&"0.03".to_owned()).parse().unwrap();
    let yearly_asset_appreciation: f32 = args.get(3).unwrap_or(&"0.10".to_owned()).parse().unwrap();
    let events: Vec<Event> = utils::read_file(format!("./{}/events.csv", folder));
    let cashflow: Vec<Cashflow> = utils::read_file(format!("./{}/cashflow.csv", folder));
    let portfolio: Vec<Asset> = utils::read_file(format!("./{}/portfolio.csv", folder));
    let mut std_wtr = csv::Writer::from_writer(io::stdout());

    let monthly_outputs: Vec<MonthlyOutput> = engine::run_engine(20, yearly_debt_interest_rate, yearly_asset_appreciation, cashflow, events, portfolio);

    for month in monthly_outputs {
        std_wtr.serialize(month).unwrap();
    }
    std_wtr.flush().unwrap()
}
