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