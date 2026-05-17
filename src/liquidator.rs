use crate::engine::RiskEngine;
use crate::position::PositionState;
use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

pub fn liquidator(engine: Arc<(Mutex<RiskEngine>, Condvar, Condvar)>) {
    let (lock, _, liq_cond) = &*engine;

    loop {
        let mut eng = lock.lock().unwrap();
        eng = liq_cond.wait_while(eng, |e| e.liq_queue.is_empty()).unwrap();

        let idx = eng.liq_queue.pop_front().unwrap() as usize;
        let pos = &mut eng.positions[idx];
        println!(
            "Liquidating trader : Market {} Size {}",
            pos.margin, pos.size
        );
        pos.state = PositionState::CLOSED;
    }
}
