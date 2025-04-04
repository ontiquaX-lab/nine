use super::{StrategyConfig, TradingStrategy};
use anyhow::Result;
use n9_exchange::tools::{Order, OrderType, Price};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TrendStrategyConfig {
    pub base: StrategyConfig,
    pub short_window: usize,
    pub long_window: usize,
    pub threshold: f64,
}

pub struct TrendStrategy {
    config: TrendStrategyConfig,
    prices: Vec<f64>,
}

impl TrendStrategy {
    pub fn new(config: TrendStrategyConfig) -> Self {
        Self {
            config,
            prices: Vec::new(),
        }
    }

    fn calculate_sma(&self, window: usize) -> Option<f64> {
        if self.prices.len() < window {
            return None;
        }
        Some(self.prices.iter().rev().take(window).sum::<f64>() / window as f64)
    }
}

#[async_trait::async_trait]
impl TradingStrategy for TrendStrategy {
    async fn execute(&self, price: Price) -> Result<Option<Order>> {
        let current_price = price.ticker.parse::<f64>()?;
        let mut prices = self.prices.clone();
        prices.push(current_price);

        let short_sma = self.calculate_sma(self.config.short_window);
        let long_sma = self.calculate_sma(self.config.long_window);

        match (short_sma, long_sma) {
            (Some(short), Some(long)) if short > long * (1.0 + self.config.threshold) => {
                Ok(Some(Order {
                    ticker: price.ticker,
                    price: None, // Market order
                    amount: self.config.base.max_position_size,
                    order_type: OrderType::Buy,
                }))
            }
            (Some(short), Some(long)) if short < long * (1.0 - self.config.threshold) => {
                Ok(Some(Order {
                    ticker: price.ticker,
                    price: None, // Market order
                    amount: self.config.base.max_position_size,
                    order_type: OrderType::Sell,
                }))
            }
            _ => Ok(None),
        }
    }

    fn config(&self) -> &StrategyConfig {
        &self.config.base
    }
}
