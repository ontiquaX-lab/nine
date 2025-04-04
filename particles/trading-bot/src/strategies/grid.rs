use super::{StrategyConfig, TradingStrategy};
use anyhow::Result;
use n9_exchange::tools::{Order, OrderType, Price};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GridConfig {
    pub base: StrategyConfig,
    pub upper_price: f64,
    pub lower_price: f64,
    pub grid_levels: usize,
    pub take_profit: f64,
    pub stop_loss: f64,
}

pub struct GridStrategy {
    config: GridConfig,
    active_orders: Vec<String>,
    current_level: usize,
}

impl GridStrategy {
    pub fn new(config: GridConfig) -> Self {
        Self {
            config,
            active_orders: Vec::new(),
            current_level: 0,
        }
    }

    fn calculate_level_prices(&self) -> Vec<f64> {
        let price_step = (self.config.upper_price - self.config.lower_price) / self.config.grid_levels as f64;
        (0..=self.config.grid_levels)
            .map(|i| self.config.lower_price + price_step * i as f64)
            .collect()
    }
}

#[async_trait::async_trait]
impl TradingStrategy for GridStrategy {
    async fn execute(&self, price: Price) -> Result<Option<Order>> {
        let current_price = price.ticker.parse::<f64>()?;
        let levels = self.calculate_level_prices();
        
        // Check if price crossed any grid level
        for (i, level) in levels.iter().enumerate() {
            if (self.current_level < i && current_price >= *level) ||
               (self.current_level > i && current_price <= *level) {
                self.current_level = i;
                let order_type = if i > self.current_level { OrderType::Sell } else { OrderType::Buy };
                return Ok(Some(Order {
                    ticker: price.ticker,
                    price: Some(*level),
                    amount: self.config.base.max_position_size / self.config.grid_levels as f64,
                    order_type,
                }));
            }
        }

        Ok(None)
    }

    fn config(&self) -> &StrategyConfig {
        &self.config.base
    }
}
