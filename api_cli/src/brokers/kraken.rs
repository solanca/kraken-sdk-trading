use kraken_rest_client::{Client as RestClient};
use kraken_rest_client::api::get_ohlc_data::{Interval};
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

    dotenv().ok();

    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY not set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET not set");

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_account_balance();
    match request.send().await {
        Ok(account_balance) => {
            println!("Account balance retrieved successfully:");
            for (key, value) in account_balance {
                println!("{}: {}", key, value);
            }
        }
        Err(error) => {
            eprintln!("Error retrieving account balance: {:?}", error);
        }
    }

}

// pub struct GetTradeBalanceResponse {
//     pub equivalent_balance: String,
//     pub trade_balance: String,
//     pub margin: String,
//     pub unrealized_net_pnl: String,
//     pub cost_basis: String,
//     pub valuation: String,
//     pub equity: String,
//     pub free_margin: String,
//     pub margin_level: Option<String>,
// }

pub async fn get_trade_balance() {
    println!("Testing get_trade_balance...");

    dotenv().ok();

    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY not set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET not set");

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_trade_balance();

    match request.send().await {
        Ok(trade_balance) => {
            println!("Trade balance retrieved successfully:");
            println!("Equivalent balance: {}", trade_balance.equivalent_balance);
            println!("Trade balance: {}", trade_balance.trade_balance);
            println!("Margin: {}", trade_balance.margin);
            println!("Unrealized net profit/loss: {}", trade_balance.unrealized_net_pnl);
            println!("Cost basis: {}", trade_balance.cost_basis);
            println!("Valuation: {}", trade_balance.valuation);
            println!("Equity: {}", trade_balance.equity);
            println!("Free margin: {}", trade_balance.free_margin);
            if let Some(margin_level) = trade_balance.margin_level {
                println!("Margin level: {}", margin_level);
            }
        }
        Err(error) => {
            eprintln!("Error retrieving trade balance: {:?}", error);
        }
    }
}


// pub struct GetTradesHistoryResponse {
//     pub trades: HashMap<String, TradeInfo>,
//     pub count: usize,
// }

// pub struct TradeInfo {
//     pub ordertxid: String,
//     pub pair: String,
//     pub time: f64,
//     pub orderside: String,
//     pub ordertype: String,
//     pub price: String,
//     pub cost: String,
//     pub fee: String,
//     pub vol: String,
//     pub margin: String,
//     pub misc: String,
// }

pub async fn get_trades_history() {
    println!("Testing get_trades_history...");

    dotenv().ok();

    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY not set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET not set");

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_trades_history();

    match request.send().await {
        Ok(trades_history) => {
            println!("Trades history retrieved successfully:");
            for (trade_id, trade) in &trades_history.trades {
                println!("Trade ID: {}", trade_id);
                println!("  Order TX ID: {}", trade.ordertxid);
                println!("  Pair: {}", trade.pair);
                println!("  Time: {}", trade.time);
                println!("  Type: {}", trade.orderside);
                println!("  Order Type: {}", trade.ordertype);
                println!("  Price: {}", trade.price);
                println!("  Cost: {}", trade.cost);
                println!("  Fee: {}", trade.fee);
                println!("  Volume: {}", trade.vol);
                println!("  Margin: {}", trade.margin);
                println!("  Misc: {}", trade.misc);
                println!();
            }
            println!("Count: {}", trades_history.count);
        }
        Err(error) => {
            eprintln!("Error retrieving trades history: {:?}", error);
        }
    }
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

    dotenv().ok();

    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY not set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET not set");

    let client = RestClient::new(api_key, api_secret);

    // Replace with the actual transaction IDs you want to query
    let txids = "TXID1,TXID2,TXID3";

    let request = client
        .query_orders_info(txids)
        .trades(true);

    match request.send().await {
        Ok(orders_info) => {
            println!("Orders information retrieved successfully:");
            for (txid, order_info) in orders_info {
                println!("Transaction ID: {}", txid);
                println!("  Userref: {:?}", order_info.userref);
                println!("  Status: {}", order_info.status);
                println!("  Order description:");
                println!("    Pair: {}", order_info.descr.pair);
                println!("    Type: {}", order_info.descr.orderside);
                println!("    Order type: {}", order_info.descr.ordertype);
                println!("    Price: {}", order_info.descr.price);
                println!("    Price 2: {}", order_info.descr.price2);
                println!("    Leverage: {}", order_info.descr.leverage);
                println!("    Order: {}", order_info.descr.order);
                println!("    Close: {}", order_info.descr.close);
                println!("  Open flags: {}", order_info.oflags);
                println!("  Open time: {}", order_info.opentm);
                println!("  Expire time: {}", order_info.expiretm);
                println!("  Volume: {}", order_info.vol);
                println!("  Volume executed: {}", order_info.vol_exec);
                println!("  Cost: {}", order_info.cost);
                println!("  Fee: {}", order_info.fee);
                println!("  Misc: {}", order_info.misc);
                println!("  Limit price: {}", order_info.limitprice);
                println!("  Reference ID: {:?}", order_info.refid);
                println!("  Reason: {:?}", order_info.reason);
                println!();
            }
        }
        Err(error) => {
            eprintln!("Error retrieving orders information: {:?}", error);
        }
    }
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


    dotenv().ok();

    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY not set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET not set");

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_system_status();

    match request.send().await {
        Ok(status) => {
            println!("System status retrieved successfully:");
            println!("Status: {}", status.status);
            println!("Timestamp: {}", status.timestamp);
        }
        Err(error) => {
            eprintln!("Error retrieving system status: {:?}", error);
        }
    }
}


pub async fn get_server_time() {
    println!("Testing get_server_time...");


    dotenv().ok();

    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY not set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET not set");

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_server_time();

    match request.send().await {
        Ok(time) => {
            println!("Server time retrieved successfully:");
            println!("Unix time: {}", time.unixtime);
            println!("RFC 1123 time: {}", time.rfc1123);
        }
        Err(error) => {
            eprintln!("Error retrieving server time: {:?}", error);
        }
    }
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


// pub struct AssetPair {
//     pub altname: String,
//     pub base: String,
//     pub quote: String,
//     pub lot_decimals: u32,
//     pub pair_decimals: u32,
//     pub lot_multiplier: i32,
//     pub leverage_buy: Vec<u32>,
//     pub leverage_sell: Vec<u32>,
//     pub fees: Vec<(u64, f64)>,
//     pub fees_maker: Vec<(u64, f64)>,
//     pub fee_volume_currency: String,
//     pub margin_call: u32,
//     pub margin_stop: u32,
//     pub ordermin: String,
// }


pub async fn get_asset_pairs() {
    println!("Testing get_asset_pairs...");

    let client = RestClient::default();

    let request = client.get_asset_pairs();

    match request.send().await {
        Ok(asset_pairs) => {
            println!("Asset pairs retrieved successfully:");
            for (pair, info) in asset_pairs {
                println!("Pair: {}", pair);
                println!("  Altname: {}", info.altname);
                println!("  Base: {}", info.base);
                println!("  Quote: {}", info.quote);
                println!("  Lot decimals: {}", info.lot_decimals);
                println!("  Pair decimals: {}", info.pair_decimals);
                println!("  Lot multiplier: {}", info.lot_multiplier);
                println!("  Leverage buy: {:?}", info.leverage_buy);
                println!("  Leverage sell: {:?}", info.leverage_sell);
                println!("  Fees: {:?}", info.fees);
                println!("  Fees maker: {:?}", info.fees_maker);
                println!("  Fee volume currency: {}", info.fee_volume_currency);
                println!("  Margin call: {}", info.margin_call);
                println!("  Margin stop: {}", info.margin_stop);
                match &info.ordermin {
                    Some(min) => println!("  Order min: {}", min),
                    None => println!("  Order min: None"),
                }
                println!();
            }
        }
        Err(error) => {
            eprintln!("Error retrieving asset pairs: {:?}", error);
        }
    }
}

pub async fn get_deposit_methods() {
    println!("Testing get_deposit_methods...");
}

fn print_usage() {
    eprintln!("List of available functions for Kraken:");
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