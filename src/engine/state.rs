use crate::market::Market;
use crate::position::Position;
use std::sync::Condvar;
use std::sync::Mutex;

#[derive(Debug)]
pub struct RiskEngine {
    //positions
    pub position: Vec<Position>,
    pub position_count: u32,
    pub position_cap: u32,

    //market
    pub market: Vec<Market>,
    pub market_count: u32,
    pub market_cap: u32,

    pub liq_queue: Vec<u32>,
    pub liq_queue_count: u32,
    pub liq_queue_cap: u32,

    pub lock: Mutex<()>,
    pub price_cond: Condvar,
    pub liq_cond: Condvar,
}

impl RiskEngine {
    pub fn engine_init(engine: RiskEngine) {}
    pub fn engine_add_position(engine: RiskEngine, position: Position) {}
    pub fn engine_add_market(engine: RiskEngine, market: Market) {}
}
