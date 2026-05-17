use crate::position::PositionState;
use crate::{engine::RiskEngine, position::Side};
use std::sync::{Arc, Condvar, Mutex};

pub fn scanner(engine: Arc<(Mutex<RiskEngine>, Condvar)>) {
    let (lock, cond) = &*engine;

    loop {
        let mut eng = lock.lock().unwrap();
        eng = cond.wait(eng).unwrap();

        for i in 0..eng.positions.len() {
            // skip non - open positions
            if eng.positions[i].state != PositionState::OPEN {
                continue;
            }

            let liquidate = {
                let pos = &eng.positions[i];
                let market = &eng.markets[pos.market_index as usize];

                match pos.side {
                    Side::LONG => market.market_price <= pos.liquidation_price,
                    Side::SHORT => market.market_price >= pos.liquidation_price,
                }
            };

            if liquidate {
                eng.positions[i].state = PositionState::LIQUIDATING;
                eng.liq_queue.push_back(i as u32);
            }
        }
    }
}
