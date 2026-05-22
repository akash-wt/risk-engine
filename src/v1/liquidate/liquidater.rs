pub fn is_liquidatable() -> bool {
    // The minimum margin ratio required to keep a position open.
    // If equity falls below this % of position value → liquidation triggered.
    // e.g. 0.5 = 50% maintenance requirement
    let maintenance_margin_rate: f32 = 0.5;

    // The price at which the position was originally opened.
    // Used to calculate unrealized PnL and position size.
    let entry_price: f32 = 20.0;

    // Collateral the trader deposited to open the position.
    // Higher leverage = less initial margin needed for same position size.
    let initial_margin: f32 = 12.0;

    // Multiplier on your margin to determine position size.
    // e.g. 2x leverage on $12 margin = $24 position.
    // Note: your current value (0.5) means 0.5x — less than 1x, which is unusual.
    let leverage: f32 = 0.5;

    // Current market price of the asset (from oracle/price feed).
    // Used to calculate unrealized PnL in real time.
    // In production this comes from Pyth/Switchboard, not hardcoded.
    let market_price: f32 = 2.0;

    let funding_accruals; 

    // Controlled asset/
    let position_size = (initial_margin * leverage) / entry_price;
    // Profit/loss based on current price vs entry
    let unrealized_PnL = (market_price - entry_price) * position_size;
    // Your current equity = what you put in ± PnL
    let equity = initial_margin + unrealized_PnL;
    // Minimum margin required to keep position open
    let maintenance_margin = position_size * entry_price * maintenance_margin_rate;
    // Liquidate if equity has fallen below maintenance threshold
    equity <= maintenance_margin
}
