#[derive(Debug, Clone, Copy)]
pub enum PositionSide {
    LONG,
    SHORT,
}

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum PositionState {
    OPEN,
    LIQUIDATING,
    CLOSED,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub entry_prcie: u64,
    pub size: u64,
    pub margin: u64,
    pub liquidation_price: u64,
    pub trade_id: [u8; 32],
    pub market_index: u32,
    pub open_at: u64,
    pub state: PositionState,
    pub side: PositionSide,
}
