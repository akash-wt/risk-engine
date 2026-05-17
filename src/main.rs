use std::{sync::{Arc, Condvar, Mutex}, thread};
use crate::engine::RiskEngine;
use liquidator::liquidator;

mod engine;
mod liquidator;
mod market;
mod position;
mod scanner;

fn main() {
    let engine = Arc::new((Mutex::new(RiskEngine::new()), Condvar::new()));
    let engine_clone = Arc::clone(&engine);
     thread::spawn(move || liquidator(engine_clone));
}
