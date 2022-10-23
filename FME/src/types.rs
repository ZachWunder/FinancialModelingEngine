use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub month: u32,
    pub name: String,
    pub cashflow: f32,
    pub debt: f32,
    pub asset: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cashflow {
    pub month: u32,
    pub name: String,
    pub amount: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub month: u32,
    pub name: String,
    pub value: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Debt {
    pub month: u32,
    pub amount: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonthlyOutput {
    pub month: u32,
    pub portfolio_sum: i64,
    pub debt_sum: i64,
    pub cashflow_sum: i64
}

pub type Portfolio = Vec<Asset>;