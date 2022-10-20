mod utils;
mod types;

use types::{Event, Cashflow, Asset};

const YEARS_TO_RUN: u32 = 70;
// 8% divided monthly
const MONTHLY_DEBT_INTEREST_RATE: f32 = 0.08 / 12.0;

fn main() {
    let events: Vec<Event> = utils::read_file("./inputs/events.csv");
    let mut cashflow: Vec<Cashflow> = utils::read_file("./inputs/cashflow.csv");
    let mut portfolio: Vec<Asset> = utils::read_file("./inputs/portfolio.csv");
    println!("{:?}", events);
    println!("{:?}", cashflow);
    println!("{:?}", portfolio);

    // Engine Loop
    for month in 0..YEARS_TO_RUN {
        // Handle Events
        let current_events: Vec<&Event> = events.iter().filter(|event| event.month == month).collect();
        println!("{:?}", current_events);
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
                    let interest_payment = current_event.debt * MONTHLY_DEBT_INTEREST_RATE;
                    // Add interest payments (negative) to cashflow
                    cashflow.push( Cashflow { month: month, name: current_event.name.clone(), amount: -interest_payment });
                }
            }
        }
        // TODO
        // Add monthly gains to portfolio
        // TODO
        // Divert excess cashflow to portfolio (securities)
    }
}

// Owned option for event handling
// let current_events: Vec<Event> = events.clone().into_iter().filter(|event| event.month == month).collect();