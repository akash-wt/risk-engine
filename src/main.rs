use crate::feed::feed;
use crate::liquidator::liquidator;
use crate::market::{Market, MarketConfig};
use crate::scanner::scanner;
use crate::{engine::RiskEngine, position::Position};
use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

mod engine;
mod feed;
mod liquidator;
mod market;
mod position;
mod scanner;
mod v1;

fn main() {
    let mut engine = RiskEngine::new();

    let sol_prep = Market {
        market_price: 100,
        config: MarketConfig {
            market_index: 0,
            maintenance_margin_rate: 5,
            initial_margin_rate: 15,
            max_leverage: 20,
        },
        last_funding_time: 0,
        funding_rate: 0,
        long_open_intrest: 0,
        short_open_intrest: 0,
    };
    engine.engine_add_market(sol_prep);

    let pos = Position {
        entry_prcie: 2_000_000,
        size: 10,
        margin: 100,
        liquidation_price: 1_800_000,
        trade_id: [0u8; 32],
        market_index: 0,
        open_at: 0,
        state: position::PositionState::OPEN,
        side: position::PositionSide::LONG,
    };

    engine.engine_add_position(pos);

    let engine = Arc::new((Mutex::new(engine), Condvar::new(), Condvar::new()));

    let e1 = Arc::clone(&engine);
    let e2 = Arc::clone(&engine);
    let e3 = Arc::clone(&engine);

    let liquidator_thread = thread::spawn(move || liquidator(e1));
    let scanner_thread = thread::spawn(move || scanner(e2));
    let feed_thread = thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(feed(e3));
    });

    liquidator_thread.join().unwrap();
    scanner_thread.join().unwrap();
    feed_thread.join().unwrap();
}
