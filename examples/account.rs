//! Fetches private account data over REST.
//!
//! Set `BITVAVO_API_KEY` and `BITVAVO_API_SECRET` in the environment
//! or in a `.env` file, then run with `cargo run --example account`.

use bitvavo_api_client::rest::RestClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = RestClient::from_env()?;

    let account = client.account().await?;
    println!(
        "Fees: taker {} maker {}",
        account.fees.taker.unwrap_or_default(),
        account.fees.maker.unwrap_or_default()
    );

    let balances = client.balances().await?;
    println!("Balances:");
    for balance in balances.iter().filter(|b| b.available != "0") {
        println!(
            "  {}: available {} in order {}",
            balance.symbol, balance.available, balance.in_order
        );
    }

    let open_orders = client.open_orders(None).await?;
    println!("Open orders: {}", open_orders.len());
    for order in &open_orders {
        println!(
            "  {} {} {:?} {:?} @ {:?}",
            order.market, order.status, order.side, order.amount, order.price
        );
    }

    Ok(())
}
