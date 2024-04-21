use kraken_rest_client::{Client as RestClient, Result as RestResult};
use kraken_rest_client::api::get_ohlc_data::{Interval, GetOhlcDataResponse};
use kraken_rest_client::types::pair_name::PairName;

use std::env;
use dotenv::dotenv;

pub async fn get_public_ohlc_data() {
    println!("Testing get_public_ohlc_data...");

    let client = RestClient::default();
    let pair = PairName::from("XBT", "USD");
    let interval = Interval::Min1;

    let request = client
        .get_ohlc_data(&pair)
        .interval(interval);

    match request.send().await {
        Ok(ohlc_data) => {
            println!("OHLC data retrieved successfully:");
            for data in ohlc_data {
                println!(
                    "Time: {}, Open: {}, High: {}, Low: {}, Close: {}, VWAP: {}, Volume: {}, Count: {}",
                    data.time(),
                    data.high(),
                    data.low(),
                    data.open(),
                    data.close(),
                    data.vwap(),
                    data.volume(),
                    data.count()
                );
            }
        }
        Err(error) => {
            eprintln!("Error retrieving OHLC data: {:?}", error);
        }
    }
}

pub async fn get_ohlc_data() {
    println!("Testing get_ohlc_data...");


    dotenv().ok();

    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY not set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET not set");


    let client = RestClient::new(api_key, api_secret);
    let pair = PairName::from("XBT", "USD");
    let interval = Interval::Min1;

    let request = client
        .get_ohlc_data(&pair)
        .interval(interval);

    match request.send().await {
        Ok(ohlc_data) => {
            println!("OHLC data retrieved successfully:");
            for data in ohlc_data {
                println!(
                    "Time: {}, Open: {}, High: {}, Low: {}, Close: {}, VWAP: {}, Volume: {}, Count: {}",
                    data.time(),
                    data.open(),
                    data.high(),
                    data.low(),
                    data.close(),
                    data.vwap(),
                    data.volume(),
                    data.count()
                );
            }
        }
        Err(error) => {
            eprintln!("Error retrieving OHLC data: {:?}", error);
        }
    }
}

pub async fn get_account_balance() {
    println!("Testing get_account_balance...");
}

pub async fn get_trade_balance() {
    println!("Testing get_trade_balance...");
}

pub async fn get_trades_history() {
    println!("Testing get_trades_history...");
}

pub async fn get_web_sockets_token() {
    println!("Testing get_web_sockets_token...");
}

pub async fn get_withdrawal_addresses() {
    println!("Testing get_withdrawal_addresses...");
}

pub async fn unstake_asset() {
    println!("Testing unstake_asset...");
}

pub async fn get_trade_volume() {
    println!("Testing get_trade_volume...");
}

pub async fn query_orders_info() {
    println!("Testing query_orders_info...");
}

pub async fn stake_asset() {
    println!("Testing stake_asset...");
}

pub async fn withdraw() {
    println!("Testing withdraw...");
}

pub async fn get_withdrawal_methods() {
    println!("Testing get_withdrawal_methods...");
}

pub async fn get_open_positions() {
    println!("Testing get_open_positions...");
}

pub async fn get_system_status() {
    println!("Testing get_system_status...");
}

pub async fn get_server_time() {
    println!("Testing get_server_time...");
}

pub async fn get_open_orders() {
    println!("Testing get_open_orders...");
}

pub async fn get_recent_trades() {
    println!("Testing get_recent_trades...");
}

pub async fn get_ledgers() {
    println!("Testing get_ledgers...");
}

pub async fn get_stakeable_assets() {
    println!("Testing get_stakeable_assets...");
}

pub async fn get_order_book() {
    println!("Testing get_order_book...");
}

pub async fn get_tickers() {
    println!("Testing get_tickers...");
}

pub async fn cancel_order_batch() {
    println!("Testing cancel_order_batch...");
}

pub async fn add_order() {
    println!("Testing add_order...");
}

pub async fn cancel_all_orders() {
    println!("Testing cancel_all_orders...");
}

pub async fn cancel_order() {
    println!("Testing cancel_order...");
}

pub async fn get_closed_orders() {
    println!("Testing get_closed_orders...");
}

pub async fn get_assets() {
    println!("Testing get_assets...");
}

pub async fn get_asset_pairs() {
    println!("Testing get_asset_pairs...");
}

pub async fn get_deposit_methods() {
    println!("Testing get_deposit_methods...");
}
