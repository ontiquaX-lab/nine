use super::{StrategyConfig, TradingStrategy};
use anyhow::Result;
use n9_exchange::tools::{Order, OrderType, Price};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SniperConfig {
    pub base: StrategyConfig,
    pub target_tokens: Vec<String>,
    pub max_slippage: f64,
    pub gas_price_limit: f64,
    pub max_retries: usize,
}

pub struct SniperStrategy {
    config: SniperConfig,
    new_listings: Vec<String>,
}

impl SniperStrategy {
    pub fn new(config: SniperConfig) -> Self {
        Self {
            config,
            new_listings: Vec::new(),
        }
    }

    fn is_target_token(&self, token: &str) -> bool {
        self.config.target_tokens.iter().any(|t| t == token) ||
        self.new_listings.iter().any(|t| t == token)
    }
}

#[async_trait::async_trait]
impl TradingStrategy for SniperStrategy {
    async fn execute(&self, price: Price) -> Result<Option<Order>> {
        if self.is_target_token(&price.ticker) {
            return Ok(Some(Order {
                ticker: price.ticker.clone(),
                price: None, // Market order for fastest execution
                amount: self.config.base.max_position_size,
                order_type: OrderType::Buy,
            }));
        }
        Ok(None)
    }

    fn config(&self) -> &StrategyConfig {
        &self.config.base
    }
}
