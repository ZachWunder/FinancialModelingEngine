use crate::{Cashflow, Portfolio, Event, types::MonthlyOutput, Asset, Debt};
use crate::{utils};

pub fn run_engine(years_to_run: u32, mut cashflow: Vec<Cashflow>, events: Vec<Event>, mut assets: Portfolio ) -> Vec<MonthlyOutput> {
  let months_to_run: u32 = years_to_run * 12;
  // 8% divided monthly
  const MONTHLY_DEBT_INTEREST_RATE: f32 = 0.08 / 12.0;
  // 10% divided monthly
  const MONTHLY_ASSET_APPRECIATION: f32 = 0.10 / 12.0;

  let mut debt: Vec<Debt> = Vec::new();
  let mut monthly_outputs: Vec<MonthlyOutput> = Vec::new();

  // Engine Loop
  for month in 0..months_to_run {
      // Handle Events
      let current_events: Vec<&Event> = events.iter().filter(|event| event.month == month).collect();
      if !current_events.is_empty() {
          for current_event in current_events {
              // Cashflow
              cashflow.push( Cashflow { month: month, name: current_event.name.clone(), amount: current_event.cashflow });
              // Assets
              if current_event.asset {
                  assets.push( Asset { month: month, name: current_event.name.clone(), value: current_event.debt });
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
      
      // Add monthly gains to assets
      let mut total_portfolio_value: f32 = 0.0;
      for asset in &assets {
          total_portfolio_value += asset.value
      }
      let monthly_gain = total_portfolio_value * MONTHLY_ASSET_APPRECIATION;
      assets.push( Asset { month: month, name: format!("Appreciation {}", month), value: monthly_gain });

      // Divert excess cashflow to assets (securities)
      let mut monthly_excess_cashflow:f32 = 0.0;
      for val in &cashflow {
          monthly_excess_cashflow += val.amount;
      }

      assets.push( Asset { month: month, name: format!("Excess Cashflow {}", month), value: monthly_excess_cashflow });

      let portfolio_total = utils::sum_portfolio(&assets) as i64;
      let debt_total = utils::sum_debt(&debt) as i64;
      let output = MonthlyOutput {
          month: month,
          portfolio_sum: portfolio_total,
          debt_sum: debt_total,
          cashflow_sum: monthly_excess_cashflow as i64
      };
      monthly_outputs.push(output)
  }
  return monthly_outputs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
      let income = Cashflow {
        month: 0,
        name: "Income".to_string(),
        amount: 5000.00
      };
      let mut cashflow: Vec<Cashflow> = Vec::new();
      cashflow.push(income);

      let events = Vec::new();
      let assets = Vec::new();

      let output = run_engine(5, cashflow, events, assets);
      let last_month = output.last().unwrap();
      
      assert_eq!(last_month.portfolio_sum, 387_185);
    }
    
    #[test]
    fn debt() {
      let income = Cashflow {
        month: 0,
        name: "Income".to_string(),
        amount: 5000.00
      };
      let mut cashflow: Vec<Cashflow> = Vec::new();
      cashflow.push(income);

      let buy_property = Event {
        name: "Buy Property".to_string(),
        month: 12,
        cashflow: 0f32,
        debt: 30000f32,
        asset: true
      };

      let mut events = Vec::new();
      events.push(buy_property);
      let assets = Vec::new();

      let output = run_engine(5, cashflow, events, assets);
      let last_month = output.last().unwrap();
      
      assert_eq!(last_month.cashflow_sum, 4_800);
      assert_eq!(last_month.portfolio_sum, 420_121);
    }
}