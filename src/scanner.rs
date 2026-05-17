use crate::position::PositionState;
use crate::{engine::RiskEngine, position::PositionSide};
use std::sync::{Arc, Condvar, Mutex};

pub fn scanner(engine: Arc<(Mutex<RiskEngine>, Condvar, Condvar)>) {
    let (lock, price_cond, liq_cond) = &*engine;

    loop {
        let mut eng = lock.lock().unwrap();
        eng = price_cond.wait(eng).unwrap();

        for i in 0..eng.positions.len() {
            // skip non - open positions
            if eng.positions[i].state != PositionState::OPEN {
                continue;
            }

            let liquidate = {
                let pos = &eng.positions[i];
                let market = &eng.markets[pos.market_index as usize];

                println!(
                    "checking: market_price={} liquidation_price={} side={:?}",
                    market.market_price, pos.liquidation_price, pos.side
                );

                match pos.side {
                    PositionSide::LONG => market.market_price <= pos.liquidation_price,
                    PositionSide::SHORT => market.market_price >= pos.liquidation_price,
                }
            };

            if liquidate {
                eng.positions[i].state = PositionState::LIQUIDATING;
                eng.liq_queue.push_back(i as u32);
                println!("pushed to liq_queue, notifying liquidator...");
                liq_cond.notify_one();
            }
        }
    }
}
