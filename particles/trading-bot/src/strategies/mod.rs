use anyhow::Result;
use async_trait::async_trait;
use n9_exchange::tools::{Order, Price};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

pub mod trend;
pub mod arbitrage;
pub mod grid;
pub mod sniper;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StrategyConfig {
    pub enabled: bool,
    pub risk_percentage: f64,
    pub max_position_size: f64,
}

#[async_trait]
pub trait TradingStrategy {
    async fn execute(&self, price: Price) -> Result<Option<Order>>;
    fn config(&self) -> &StrategyConfig;
}
