/*use kraken_rest_client::{Client as RestClient, Result as RestResult, *};
use kraken_futures_rest_client::{Client as FuturesClient, Result as FuturesResult};

use std::env;

fn test_get_account_balance(rest_client: &RestClient) -> RestResult<()> {
    let balance = rest_client.get_account_balance()?;
    println!("Account Balance: {:?}", balance);
    Ok(())
}

fn test_place_order(rest_client: &RestClient) -> RestResult<()> {
    let order = Order {
        pair: PairName::new("XBTUSD"),
        order_type: OrderType::Buy,
        volume: 1.0,
        price: Some(30000.0),
        ..Default::default()
    };
    let response = rest_client.add_order(&order)?;
    println!("Order Response: {:?}", response);
    Ok(())
}

fn test_get_market_data(rest_client: &RestClient) -> RestResult<()> {
    let data = rest_client.get_ohlc_data(&PairName::new("XBTUSD"), Interval::OneMinute, None, None)?;
    println!("Market Data: {:?}", data);
    Ok(())
}

fn test_futures_get_instruments(futures_client: &FuturesClient) -> FuturesResult<()> {
    let instruments = futures_client.get_instruments()?;
    println!("Futures Instruments: {:?}", instruments);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cli_tester <function_name>");
        return;
    }

    let rest_client = RestClient::new("api_key", "api_secret");
    let futures_client = FuturesClient::new("api_key", "api_secret");

    match args[1].as_str() {
        "get_account_balance" => test_get_account_balance(&rest_client).unwrap(),
        "place_order" => test_place_order(&rest_client).unwrap(),
        "get_market_data" => test_get_market_data(&rest_client).unwrap(),
        "futures_get_instruments" => test_futures_get_instruments(&futures_client).unwrap(),
        _ => println!("Unknown function: {}", args[1]),
    }
}*/