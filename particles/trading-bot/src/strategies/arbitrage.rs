use super::{StrategyConfig, TradingStrategy};
use anyhow::Result;
use n9_exchange::tools::{Order, OrderType, Price};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArbitrageConfig {
    pub base: StrategyConfig,
    pub min_profit_percentage: f64,
    pub max_slippage: f64,
    pub exchanges: Vec<String>,
}

pub struct ArbitrageStrategy {
    config: ArbitrageConfig,
    exchange_prices: std::collections::HashMap<String, f64>,
}

impl ArbitrageStrategy {
    pub fn new(config: ArbitrageConfig) -> Self {
        Self {
            config,
            exchange_prices: std::collections::HashMap::new(),
        }
    }

    fn find_arbitrage_opportunity(&self) -> Option<(String, String, f64)> {
        let mut best_buy = None;
        let mut best_sell = None;

        for (exchange, price) in &self.exchange_prices {
            if best_buy.is_none() || price < best_buy.unwrap().1 {
                best_buy = Some((exchange.clone(), *price));
            }
            if best_sell.is_none() || price > best_sell.unwrap().1 {
                best_sell = Some((exchange.clone(), *price));
            }
        }

        if let (Some((buy_ex, buy_price)), Some((sell_ex, sell_price))) = (best_buy, best_sell) {
            let profit = (sell_price - buy_price) / buy_price * 100.0;
            if profit >= self.config.min_profit_percentage && buy_ex != sell_ex {
                return Some((buy_ex, sell_ex, profit));
            }
        }
        None
    }
}

#[async_trait::async_trait]
impl TradingStrategy for ArbitrageStrategy {
    async fn execute(&self, price: Price) -> Result<Option<Order>> {
        // In a real implementation, we would compare prices across multiple exchanges
        // This is a simplified version that just tracks prices
        self.exchange_prices.insert(price.ticker.clone(), price.ticker.parse::<f64>()?);

        if let Some((buy_ex, sell_ex, profit)) = self.find_arbitrage_opportunity() {
            // In a full implementation, we would:
            // 1. Execute buy on buy_ex
            // 2. Execute sell on sell_ex
            // 3. Handle potential flash loans
            log::info!("Arbitrage opportunity found: buy on {}, sell on {} ({}% profit)", 
                      buy_ex, sell_ex, profit);
        }
        Ok(None)
    }

    fn config(&self) -> &StrategyConfig {
        &self.config.base
    }
}
