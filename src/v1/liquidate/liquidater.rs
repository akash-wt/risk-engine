use crate::v1::market::market::Market;
use crate::v1::position::position::Position;

pub fn is_liquidatable(position: Position, market: Market) -> bool {
    // The minimum margin ratio required to keep a position open.
    // If equity falls below this % of position value → liquidation triggered.

    let maintenance_margin_rate: f32 = 0.5;

    // The price at which the position was originally opened.
    // Used to calculate unrealized PnL and position size.
    let entry_price: f32 = 20.0;

    let leverage: f32 = 0.5;

    let market_price: f32 = 2.0;

    let funding_accruals;

    // Controlled asset/
    let position_size = (position.initial_margin * position.leverage) / entry_price;
    // Profit/loss based on current price vs entry
    let unrealized_PnL = (market_price - entry_price) * position_size;
    // Your current equity = what you put in ± PnL
    let equity = initial_margin + unrealized_PnL;
    // Minimum margin required to keep position open
    let maintenance_margin = position_size * entry_price * maintenance_margin_rate;
    // Liquidate if equity has fallen below maintenance threshold
    equity <= maintenance_margin
}
