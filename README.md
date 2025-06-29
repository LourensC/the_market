# the_market

A minimal order-matching engine written in Rust. The crate provides a `Market` struct to manage `BuyOrder` and `SellOrder`s and a `resolve_orders` function to match trades.

## Build and Test

```bash
cargo build
cargo test
```

## Example

```rust
use the_market::market::{Market, BuyOrder, SellOrder};

fn main() {
    let mut market = Market::new();
    market.place_buy_order(BuyOrder::new("buyer".into(), 100, 1));
    market.place_sell_order(SellOrder::new("seller".into(), 100, 1));

    let trades = market.resolve_orders().expect("no trades");
    for trade in trades {
        println!("buyer {} pays {}", trade.buyer.debit.account_id, trade.buyer.credit.amount);
        println!("seller {} receives {}", trade.seller.debit.account_id, trade.seller.credit.amount);
    }
}
```

Running the example prints the resulting trades after the engine matches the best available buy and sell orders.
