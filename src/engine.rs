use crate::market::Market;
use crate::position::Position;
use crate::position::Side;
use std::sync::Condvar;
use std::sync::Mutex;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct RiskEngine {
    //positions
    pub positions: Vec<Position>,
    pub position_count: u32,
    pub position_cap: u32,

    //market
    pub markets: Vec<Market>,
    pub market_count: u32,
    pub market_cap: u32,

    pub liq_queue: VecDeque<u32>,
    pub liq_queue_count: u32,
    pub liq_queue_cap: u32,

    pub lock: Mutex<()>,
    pub price_cond: Condvar,
    pub liq_cond: Condvar,
}

impl RiskEngine {
    pub fn new() -> Self {
        Self {
            positions: Vec::with_capacity(1000),
            position_count: 0,
            position_cap: 1000,

            markets: Vec::with_capacity(64),
            market_count: 0,
            market_cap: 64,

            liq_queue: VecDeque::with_capacity(256),
            liq_queue_count: 0,
            liq_queue_cap: 256,
            lock: Mutex::new(()),
            price_cond: Condvar::new(),
            liq_cond: Condvar::new(),
        }
    }
    pub fn engine_add_position(&mut self, position: Position) {
        let market_index = position.market_index;
        let side = position.side;
        let size = position.size;

        self.positions.push(position);

        if market_index >= self.market_count {
            return;
        }

        let market = &mut self.markets[market_index as usize];
        match side {
            Side::LONG => market.long_open_intrest += size,
            Side::SHORT => market.short_open_intrest += size,
        }
    }
    pub fn engine_add_market(&mut self, market: Market) {
        self.markets.push(market);
    }
}
