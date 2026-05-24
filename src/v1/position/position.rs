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
    pub trade_id: [u8; 32],
    pub entry_price: u64, 
    pub initial_margin: f32,
    pub leverage: f32,
    pub size: u64,
    pub open_at: u64,
    pub state: PositionState,
    pub side: PositionSide,
}
