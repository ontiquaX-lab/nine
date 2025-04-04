use anyhow::Result;
use async_trait::async_trait;
use n9_core::{Particle, SubstanceLinks};
use n9_exchange::tools::{Order, Price, Tickers};
use n9_kit::{LiquidParticle, Toolkit};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

pub mod strategies;

static TOOLKIT: &str = "TradingBot";

#[derive(Default)]
pub struct TradingBotToolkit;

#[async_trait]
impl Toolkit for TradingBotToolkit {
    async fn add_tools(
        &mut self,
        particle: &mut LiquidParticle<Self>,
        bond: &mut n9_core::SubstanceBond<LiquidParticle<Self>>,
    ) -> Result<()> {
        bond.add_tool::<Price>(particle, TOOLKIT, "Get Price")
            .await?;
        bond.add_tool::<Tickers>(particle, TOOLKIT, "List Tickers")
            .await?;
        bond.add_tool::<Order>(particle, TOOLKIT, "Place Order")
            .await?;
        Ok(())
    }
}

pub type TradingBotParticle = LiquidParticle<TradingBotToolkit>;

impl Particle for TradingBotParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        LiquidParticle::new(substance, TradingBotToolkit::default())
    }
}
