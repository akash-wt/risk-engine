#[derive(Debug, Clone)]
struct MarketConfig {
    market_index: u32,
    initial_margin_rate: u64,
    maintenance_margin_rate: u64,
}

#[derive(Debug, Clone)]
pub struct Market {
    market_price: u64,
    long_open_intrest: u64,
    short_open_intrest: u64,
    funding_rate: u64,
    last_funding_time: u64,
    config: MarketConfig,
}
