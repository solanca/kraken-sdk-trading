/*use kraken_rest_client::{Client as RestClient, Result as RestResult, *};
use kraken_futures_rest_client::{Client as FuturesClient, Result as FuturesResult};

use std::env;

async fn test_get_account_balance(rest_client: &RestClient) -> RestResult<()> {
    let balance = rest_client.get_account_balance().send().await?;
    println!("Account Balance: {:?}", balance);
    Ok(())
}

async fn test_place_order(rest_client: &RestClient) -> RestResult<()> {
    let order = AddOrderRequest::limit(
        OrderSide::Buy,
        1.0,
        &PairName::from("XBT", "USD"), 
        30000.0,
    );
    let response = rest_client.add_order(order).send().await?;
    println!("Order Response: {:?}", response);
    Ok(())
}

async fn test_get_market_data(rest_client: &RestClient) -> RestResult<()> {
    let data = rest_client
        .get_ohlc_data(&PairName::from("XBT", "USD"))
        .interval(Interval::Min1)
        .send()
        .await?;
    println!("Market Data: {:?}", data);
    Ok(())
}

async fn test_futures_get_instruments(futures_client: &FuturesClient) -> FuturesResult<()> {
    let instruments = futures_client.get_instruments().send().await?;
    println!("Futures Instruments: {:?}", instruments);
    Ok(())
}


fn test_get_asset_pairs(rest_client: &RestClient) -> RestResult<()> {
    let assets = rest_client.get_asset_pairs()?;
    println!("Tradable Assets: {:?}", assets);
    Ok(())
}

fn test_get_ticker_info(rest_client: &RestClient) -> RestResult<()> {
    let ticker_info = rest_client.get_ticker(&["XBTUSD", "ETHUSD"])?;
    println!("Ticker Information: {:?}", ticker_info);
    Ok(())
}

fn test_get_server_time(rest_client: &RestClient) -> RestResult<()> {
    let server_time = rest_client.get_server_time()?;
    println!("Server Time: {:?}", server_time);
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cli_tester <function_name>");
        return;
    }

    let rest_client = RestClient::new("api_key", "api_secret");

    match args[1].as_str() {
        "get_account_balance" => test_get_account_balance(&rest_client).await.unwrap(),
        "place_order" => test_place_order(&rest_client).unwrap(),
        "get_market_data" => test_get_market_data(&rest_client).unwrap(),
        "futures_get_instruments" => test_futures_get_instruments(&futures_client).await.unwrap(),
        "get_asset_pairs" => test_get_asset_pairs(&rest_client).unwrap(),
        "get_ticker_info" => test_get_ticker_info(&rest_client).unwrap(),
        "get_server_time" => test_get_server_time(&rest_client).unwrap(),
        _ => println!("Unknown function: {}", args[1]),
    }
}





*/