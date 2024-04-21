use kraken_rest_client::{Client as RestClient, Result as RestResult};
use kraken_rest_client::api::get_ohlc_data::{Interval, GetOhlcDataResponse};
use kraken_rest_client::types::pair_name::PairName;

use std::env;
use dotenv::dotenv;

pub async fn handle_function(function_name: &str) {
    match function_name {
        "get_account_balance" => get_account_balance().await,
        "get_trade_balance" => get_trade_balance().await,
        "get_trades_history" => get_trades_history().await,
        "get_web_sockets_token" => get_web_sockets_token().await,
        "get_withdrawal_addresses" => get_withdrawal_addresses().await,
        "unstake_asset" => unstake_asset().await,
        "get_trade_volume" => get_trade_volume().await,
        "query_orders_info" => query_orders_info().await,
        "stake_asset" => stake_asset().await,
        "withdraw" => withdraw().await,
        "get_withdrawal_methods" => get_withdrawal_methods().await,
        "get_open_positions" => get_open_positions().await,
        "get_system_status" => get_system_status().await,
        "get_server_time" => get_server_time().await,
        "get_open_orders" => get_open_orders().await,
        "get_recent_trades" => get_recent_trades().await,
        "get_ohlc_data" => get_ohlc_data().await,
        "get_public_ohlc_data" => get_public_ohlc_data().await,
        "get_ledgers" => get_ledgers().await,
        "get_stakeable_assets" => get_stakeable_assets().await,
        "get_order_book" => get_order_book().await,
        "get_tickers" => get_tickers().await,
        "cancel_order_batch" => cancel_order_batch().await,
        "add_order" => add_order().await,
        "cancel_all_orders" => cancel_all_orders().await,
        "cancel_order" => cancel_order().await,
        "get_closed_orders" => get_closed_orders().await,
        "get_assets" => get_assets().await,
        "get_asset_pairs" => get_asset_pairs().await,
        "get_deposit_methods" => get_deposit_methods().await,
        _ => {
            eprintln!("Function '{}' not recognized for Kraken.", function_name);
            print_usage();
            std::process::exit(1);
        }
    }
}

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

fn print_usage() {
    eprintln!("List of available functions for Coinbase:");
    eprintln!("  get_account_balance - Test the account balance API");
    eprintln!("  get_trade_balance - Test the trade balance API");
    eprintln!("  get_trades_history - Test trades history API");
    eprintln!("  get_web_sockets_token - Test WebSocket token retrieval");
    eprintln!("  get_withdrawal_addresses - Test withdrawal addresses retrieval");
    eprintln!("  unstake_asset - Test asset unstaking");
    eprintln!("  get_trade_volume - Test trade volume retrieval");
    eprintln!("  query_orders_info - Test orders information query");
    eprintln!("  stake_asset - Test asset staking");
    eprintln!("  withdraw - Test withdrawing funds");
    eprintln!("  get_withdrawal_methods - Test withdrawal methods retrieval");
    eprintln!("  get_open_positions - Test open positions retrieval");
    eprintln!("  get_system_status - Test system status check");
    eprintln!("  get_server_time - Test server time check");
    eprintln!("  get_open_orders - Test open orders retrieval");
    eprintln!("  get_recent_trades - Test recent trades retrieval");
    eprintln!("  get_ohlc_data - Test OHLC data retrieval");
    eprintln!("  get_public_ohlc_data - Test public OHLC data retrieval");
    eprintln!("  get_ledgers - Test ledgers retrieval");
    eprintln!("  get_stakeable_assets - Test stakeable assets retrieval");
    eprintln!("  get_order_book - Test order book retrieval");
    eprintln!("  get_tickers - Test tickers retrieval");
    eprintln!("  cancel_order_batch - Test batch order cancellation");
    eprintln!("  add_order - Test adding orders");
    eprintln!("  cancel_all_orders - Test cancellation of all orders");
    eprintln!("  cancel_order - Test single order cancellation");
    eprintln!("  get_closed_orders - Test closed orders retrieval");
    eprintln!("  get_assets - Test assets retrieval");
    eprintln!("  get_asset_pairs - Test asset pairs retrieval");
    eprintln!("  get_deposit_methods - Test deposit methods retrieval");
}