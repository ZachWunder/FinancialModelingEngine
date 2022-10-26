mod utils;
mod engine;
mod types;

use std::{env, io};
use types::{Event, Cashflow, Asset, Portfolio, Debt, MonthlyOutput};



fn main() {
    let args: Vec<String> = env::args().collect();
    let folder = &args[1];
    let events: Vec<Event> = utils::read_file(format!("./inputs/{}/events.csv", folder));
    let cashflow: Vec<Cashflow> = utils::read_file(format!("./inputs/{}/cashflow.csv", folder));
    let portfolio: Vec<Asset> = utils::read_file(format!("./inputs/{}/portfolio.csv", folder));
    let mut std_wtr = csv::Writer::from_writer(io::stdout());

    let monthly_outputs: Vec<MonthlyOutput> = engine::run_engine(70, cashflow, events, portfolio);

    for month in monthly_outputs {
        std_wtr.serialize(month).unwrap();
    }
    std_wtr.flush().unwrap()
}
