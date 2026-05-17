#[derive(Debug, Clone)]
struct MarketConfig {
    pub market_index: u32,
    pub initial_margin_rate: u64,
    pub maintenance_margin_rate: u64,
}

#[derive(Debug, Clone)]
pub struct Market {
    pub market_price: u64,
    pub long_open_intrest: u64,
    pub short_open_intrest: u64,
    pub funding_rate: u64,
    pub last_funding_time: u64,
    pub config: MarketConfig,
}
