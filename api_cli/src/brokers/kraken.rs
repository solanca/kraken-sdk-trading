use kraken_rest_client::{Client as RestClient, GetOhlcDataRequest};
use kraken_rest_client::{Interval};
use kraken_rest_client::types::pair_name::PairName;
use kraken_rest_client::OrderSide;
use kraken_rest_client::OrderType;
use tokio::runtime::Runtime;
use std::env;
use dotenv::dotenv;


fn load_api_credentials() -> (String, String) {
    dotenv().ok();
    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY not set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET not set");
    (api_key, api_secret)
}

pub fn execute_command(command: &serde_json::Value, args: &[String]) -> Result<(), String> {
    let handler = command["name"].as_str().unwrap();
    println!("All arguments received execute: {:?}", args);
    match handler {
        "get_account_balance" => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_account_balance()).expect("Failed to retrieve account balance");
            Ok(())
        }
        "get_assets" => {
            let assets = args.get(0).expect("Missing argument: assets");
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_assets(assets))
        }
        "get_asset_pairs" => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_asset_pairs())
        }
        "get_tickers" => {
            let pairs = args.get(0).expect("Missing argument: pairs");
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_tickers(pairs))
        }
        "get_ledgers" => {
            let aclass = args.get(0).map(|v| v.as_str());
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_ledgers(aclass))
        }
        "get_order_book" => {
            let pair = args.get(0).expect("Missing argument: pair");
            let count = args.get(1).map(|v| v.parse().unwrap_or(100)).unwrap_or(100);
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_order_book(pair, count))
        }
        "get_recent_trades" => {
            let pair = args.get(0).expect("Missing argument: pair");
            let since = args.get(1).map(|v| v.as_str());
            let count = args.get(2).map(|v| v.parse().ok()).flatten();
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_recent_trades(pair, since, count))
        }
        "get_trade_balance" => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_trade_balance()).expect("Failed to retrieve trade balance");
            Ok(())
        }
        "get_trade_history" => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_trade_history()).expect("Failed to retrieve trade history");
            Ok(())
        }
        "get_trade_volume" => {
            let pair = args.get(0).expect("Missing argument: pair");
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_trade_volume(pair))
        }
        "get_open_positions" => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_open_positions())
        }
        "get_open_orders" => {
            let trades = args.get(0).map(|v| v.parse().unwrap_or(false)).unwrap_or(false);
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_open_orders(trades))
        }
        "cancel_all_orders" => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(cancel_all_orders())
        }
        "cancel_order_batch" => {
            let txids = args.get(0).expect("Missing argument: txids");
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(cancel_order_batch(txids))
        }
        "cancel_order" => {
            let txid = args.get(0).expect("Missing argument: txid");
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(cancel_order(txid))
        }
        "add_order" => {
            let pair = args.get(0).expect("Missing argument: pair");
            let side = args.get(1).expect("Missing argument: side");
            let order_type = args.get(2).expect("Missing argument: type");
            let price = args.get(3).expect("Missing argument: price");
            let volume = args.get(4).expect("Missing argument: volume");
        
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(add_order(pair, side, order_type, price, volume))
        }
        "query_orders_info" => {
            let txids = args.get(0).expect("Missing argument: txids");
            let trades = args.get(1).map(|v| v.parse().unwrap_or(false)).unwrap_or(false);
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(query_orders_info(txids, trades))
        }
        "get_system_status" => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_system_status())
        }
        "get_server_time" => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_server_time())
        }
        "get_public_ohlc_data" => {
            println!("All arguments received: {:?}", args);
            let base = args.get(0).expect("Missing argument: base");
            let quote = args.get(1).expect("Missing argument: quote");
            let interval = match args.get(2).expect("Missing argument: interval").as_str() {
                "1" => Interval::Min1,
                "5" => Interval::Min5,
                "15" => Interval::Min15,
                "30" => Interval::Min30,
                "60" => Interval::Hour1,
                "240" => Interval::Hour4,
                "1440" => Interval::Day1,
                "10080" => Interval::Day7,
                "21600" => Interval::Day15,
                _ => return Err("Invalid interval. Supported intervals: 1, 5, 15, 30, 60, 240, 1440, 10080, 21600".to_string()),
            };
        
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_public_ohlc_data(base, quote, interval))
        }      
        "verify_order" => {
            let pair = args.get(0).expect("Missing argument: pair");
            let side = match args.get(1).expect("Missing argument: side").as_str() {
                "buy" => "buy",
                "sell" => "sell",
                _ => return Err("Invalid order side. Expected 'buy' or 'sell'.".to_string()),
            };
            let order_type = match args.get(2).expect("Missing argument: type").as_str() {
                "limit" => "limit",
                "market" => "market",
                // Add more order types as needed
                _ => return Err("Invalid order type.".to_string()),
            };
            let price = args.get(3).expect("Missing argument: price");
            let volume = args.get(4).expect("Missing argument: volume");
        
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(verify_order(pair, side, order_type, price, volume))
        }
        "get_closed_orders" => {
            let trades = args.get(0).map(|v| v.parse().unwrap_or(false)).unwrap_or(false);
            let start = args.get(1).map(|v| v.parse().ok()).flatten();
            let end = args.get(2).map(|v| v.parse().ok()).flatten();
            let ofs = args.get(3).map(|v| v.parse().ok()).flatten();
        
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_closed_orders(trades, start, end, ofs))
        }
        "get_deposit_addresses" => {
            let asset = args.get(0).expect("Missing argument: asset");
            let method = args.get(1).expect("Missing argument: method");
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_deposit_addresses(asset, method))
        }
        "get_deposit_methods" => {
            let asset = args.get(0).expect("Missing argument: asset");
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_deposit_methods(asset))
        }
        "get_deposit_status" => {
            let asset = args.get(0).expect("Missing argument: asset");
            let method = args.get(1).map(|v| v.as_str());
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_deposit_status(asset, method))
        }
        "get_stakeable_assets" => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(get_stakeable_assets())
        }
        "stake_asset" => {
            let asset = args.get(0).expect("Missing argument: asset");
            let amount = args.get(1).expect("Missing argument: amount");
            let method = args.get(2).expect("Missing argument: method");
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(stake_asset(asset, amount, method))
        }
        "get_web_sockets_token" => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            match rt.block_on(get_web_sockets_token()) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("{}", e);
                    Err(e)
                }
            }
        }
        _ => {
            println!("Executing {} with arguments {:?}", handler, args);
            // Simulated handler execution
            Ok(())
        }
    }
}

pub async fn get_public_ohlc_data(base: &str, quote: &str, interval: Interval) -> Result<(), String> {
    let base_upper = base.to_uppercase();
    let quote_upper = quote.to_uppercase();
    let pair = format!("{}{}", base_upper, quote_upper);
    println!("Fetching public OHLC data for pair: {}", pair);

    let client = RestClient::default();
    let pair_name = PairName::from(&base_upper, &quote_upper);
    let request = client.get_ohlc_data(&pair_name).interval(interval);

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
            Ok(())
        },
        Err(error) => {
            let error_message = format!("Error retrieving OHLC data: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}


pub async fn get_account_balance() -> Result<(), String> {
    println!("Testing get_account_balance...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_account_balance();
    match request.send().await {
        Ok(account_balance) => {
            println!("Account balance retrieved successfully:");
            for (key, value) in account_balance {
                println!("{}: {}", key, value);
            }
            Ok(())
        }
        Err(error) => {
            eprintln!("Error retrieving account balance: {:?}", error);
            Err(format!("Error retrieving account balance: {:?}", error))
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

pub async fn get_trade_balance() -> Result<(), String> {
    println!("Testing get_trade_balance...");

    let (api_key, api_secret) = load_api_credentials();

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
            Ok(())
        }
        Err(error) => {
            eprintln!("Error retrieving trade balance: {:?}", error);
            Err(format!("Error retrieving trade balance: {:?}", error))
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

pub async fn get_trade_history() -> Result<(), String> {
    println!("Testing get_trade_history...");

    let (api_key, api_secret) = load_api_credentials();

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
            Ok(())
        }
        Err(error) => {
            eprintln!("Error retrieving trades history: {:?}", error);
            Err(format!("Error retrieving trades history: {:?}", error))
        }
    }
}

//pub struct GetWebSocketsTokenResponse {
//    pub token: String,
//    pub expires: i64,
//}

pub async fn get_web_sockets_token() -> Result<(), String> {
    println!("Retrieving WebSocket token...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_web_sockets_token();

    match request.send().await {
        Ok(response) => {
            println!("WebSocket token retrieved successfully:");
            println!("Token: {}, Expires in: {} seconds", response.token, response.expires);
            Ok(())
        }
        Err(e) => {
            if e.to_string().contains("Permission denied") {
                eprintln!("Permission denied while retrieving WebSocket token: {}", e);
                Err(format!("Permission denied while retrieving WebSocket token: {}", e))
            } else {
                eprintln!("Failed to retrieve WebSocket token: {}", e);
                Err(format!("Failed to retrieve WebSocket token: {}", e))
            }
        }
    }
}


pub async fn get_withdrawal_addresses() {
    println!("Fetching withdrawal addresses...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    // Create a request to get withdrawal addresses without filters
    let request = client.get_withdrawal_addresses()
        .asset("XBT")  // Example: filter by asset, can be modified or omitted
        .aclass("currency")  // Example: filter by asset class, can be modified or omitted
        .method("Bitcoin");  // Example: filter by method, can be modified or omitted

    match request.send().await {
        Ok(addresses) => {
            println!("Withdrawal addresses retrieved successfully:");
            for address in addresses {
                println!("Asset: {}, Method: {}, Address: {}, Key: {}, Verified: {}",
                         address.asset, address.method, address.address, address.key, address.verified);
            }
        },
        Err(error) => {
            eprintln!("Error retrieving withdrawal addresses: {:?}", error);
        }
    }
}


pub async fn unstake_asset() {
    println!("Testing unstake_asset...");
}

pub async fn get_trade_volume(pair: &str) -> Result<(), String> {
    println!("Fetching trade volume for pair: {}", pair);

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_trade_volume(pair);

    match request.send().await {
        Ok(trade_volume) => {
            println!("Trade volume retrieved successfully:");
            println!("Currency: {}", trade_volume.currency);
            println!("Volume: {}", trade_volume.volume);
            println!("Fees: {:?}", trade_volume.fees);
            println!("Maker Fees: {:?}", trade_volume.fees_maker);
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error retrieving trade volume: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}




pub async fn query_orders_info(txids: &str, trades: bool) -> Result<(), String> {
    println!("Querying orders information...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);

    let request = client
        .query_orders_info(txids)
        .trades(trades);

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
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error retrieving orders information: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}

pub async fn stake_asset(asset: &str, amount: &str, method: &str) -> Result<(), String> {
    println!("Staking asset...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let request = client.stake_asset(asset, amount, method);

    match request.send().await {
        Ok(stake_result) => {
            println!("Asset staked successfully.");
            println!("Refid: {}", stake_result.refid);
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error staking asset: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}

pub async fn withdraw() {
    println!("Withdrawing funds...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let asset = "XBT"; // Example asset to withdraw
    let key = "my_withdrawal_key"; // Example withdrawal key
    let amount = "0.1"; // Example amount to withdraw

    let request = client.withdraw(asset, key, amount);

    match request.send().await {
        Ok(withdraw_result) => {
            println!("Withdrawal successful.");
            println!("Refid: {}", withdraw_result.refid);
        }
        Err(error) => {
            eprintln!("Error withdrawing funds: {:?}", error);
        }
    }
}

pub async fn get_withdrawal_status() {
    println!("Fetching withdrawal status... can you query this????");

/*  let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let asset = "XBT"; // Example asset

    let request = client.get_withdrawal_status(asset);

    match request.send().await {
        Ok(withdrawal_statuses) => {
            println!("Withdrawal statuses retrieved successfully:");
            for status in withdrawal_statuses {
                println!("Method: {}, Aclass: {}, Asset: {}, Refid: {}, TX ID: {}, Info: {}, Amount: {}, Fee: {:?}, Time: {}, Status: {:?}, Status Prop: {:?}",
                         status.method, status.aclass, status.asset, status.refid, status.txid, status.info, status.amount, status.fee, status.time, status.status, status.status_prop);
            }
        },
        Err(error) => {
            eprintln!("Error retrieving withdrawal status: {:?}", error);
        }
    }*/
}

pub async fn get_withdrawal_methods() {
    println!("Fetching withdrawal methods...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let asset = "XBT"; // Example asset

    let request = client.get_withdrawal_methods().asset(asset);

    match request.send().await {
        Ok(methods) => {
            println!("Withdrawal methods retrieved successfully:");
            for method in methods {
                println!("Method: {}", method.method);
                println!("  Asset: {}", method.asset);
                println!("  Network: {}", method.network);
                println!("  Minimum: {}", method.minimum);
                println!();
            }
        }
        Err(error) => {
            eprintln!("Error retrieving withdrawal methods: {:?}", error);
        }
    }
}

/// Retrieves the open positions from the Kraken API, always including P&L calculations.
pub async fn get_open_positions() -> Result<(), String> {
    println!("Retrieving open positions with P&L calculations...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_open_positions().docalcs(true);

    match request.send().await {
        Ok(positions) => {
            println!("Open positions retrieved successfully:");
            for (pos_id, position) in positions {
                println!(
                    "Position ID: {}, Order Tx ID: {}, Status: {}, Pair: {}, Type: {}, Order Type: {}, Cost: {}, Fee: {}, Volume: {}, Closed Volume: {}, Margin: {}",
                    pos_id,
                    position.ordertxid,
                    position.posstatus,
                    position.pair,
                    position.position_type,
                    position.ordertype,
                    position.cost,
                    position.fee,
                    position.vol,
                    position.vol_closed,
                    position.margin
                );

                if let Some(value) = &position.value {
                    println!("Value: {}", value);
                }

                if let Some(net) = &position.net {
                    println!("Net: {}", net);
                }
            }
            Ok(())
        }
        Err(e) => {
            let error_message = format!("Failed to retrieve open positions: {}", e);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}

pub async fn get_system_status() -> Result<(), String> {
    println!("Fetching system status...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_system_status();

    match request.send().await {
        Ok(status) => {
            println!("System status retrieved successfully:");
            println!("Status: {}", status.status);
            println!("Timestamp: {}", status.timestamp);
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error retrieving system status: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}


pub async fn get_server_time() -> Result<(), String> {
    println!("Fetching server time...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_server_time();

    match request.send().await {
        Ok(time) => {
            println!("Server time retrieved successfully:");
            println!("Unix time: {}", time.unixtime);
            println!("RFC 1123 time: {}", time.rfc1123);
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error retrieving server time: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}


/// Retrieves the open orders from the Kraken API.
pub async fn get_open_orders(trades: bool) -> Result<(), String> {
    println!("Retrieving open orders...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_open_orders().trades(trades);

    match request.send().await {
        Ok(response) => {
            println!("Open orders retrieved successfully:");
            for (order_id, order_info) in response.open {
                println!(
                    "Order ID: {}, Status: {}, Cost: {}, Description: Pair: {}, Side: {}, Order Type: {}, Price: {}, Price2: {}, Open Time: {}, Flags: {}, Fee: {}, Volume: {}, Executed Volume: {}",
                    order_id,
                    order_info.status,
                    order_info.cost,
                    order_info.descr.pair,
                    order_info.descr.orderside,
                    order_info.descr.ordertype,
                    order_info.descr.price,
                    order_info.descr.price2,
                    order_info.opentm,
                    order_info.oflags,
                    order_info.fee,
                    order_info.vol,
                    order_info.vol_executed.unwrap_or("0".to_string()),
                );
            }
            Ok(())
        }
        Err(e) => {
            let error_message = format!("Failed to retrieve open orders: {}", e);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}




/// Retrieves recent public trades from a unix timestamp, 0 means ALL 
pub async fn get_recent_trades(pair: &str, since: Option<&str>, count: Option<u16>) -> Result<(), String> {
    println!("Retrieving recent trades for pair: {}", pair);

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);
    let mut last_id = since.map(|s| s.to_string());
    let mut last_trade_id = None;

    loop {
        let mut request = client.get_recent_trades(pair);

        if let Some(last) = last_id.as_ref() {
            request = request.since(last.clone());
        }

        if let Some(cnt) = count {
            request = request.count(cnt);
        }

        match request.send().await {
            Ok(response) => {
                println!("Recent trades retrieved successfully:");
                for (pair_key, trades) in response.pair {
                    println!("Trading pair: {}", pair_key);
                    for trade in &trades {
                        println!(
                            "Trade ID: {}, Price: {}, Volume: {}, Time: {}, Type: {}, Order Type: {}, Misc: {}",
                            trade.trade_id(),
                            trade.price(),
                            trade.volume(),
                            trade.time(),
                            trade.buy_sell(),
                            trade.market_limit(),
                            trade.miscellaneous()
                        );

                        if let Some(last_id) = last_trade_id {
                            if last_id == trade.trade_id() {
                                return Ok(());
                            }
                        }
                        last_trade_id = Some(trade.trade_id());
                    }

                    if trades.is_empty() {
                        return Ok(());
                    }
                }

                last_id = Some(response.last);
            }
            Err(e) => {
                let error_message = format!("Failed to retrieve recent trades: {}", e);
                eprintln!("{}", error_message);
                return Err(error_message);
            }
        }
    }
}

/// Retrieves ledger entries from the Kraken API, hardcoding for currency asset class and no specific filtering.
pub async fn get_ledgers(aclass: Option<&str>) -> Result<(), String> {
    println!("Retrieving ledger entries...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let mut request = client.get_ledgers();

    if let Some(asset_class) = aclass {
        request = request.aclass(asset_class);
    }

    match request.send().await {
        Ok(response) => {
            println!("Ledger entries retrieved successfully:");
            for (entry_id, ledger_entry) in response.ledger {
                println!(
                    "Entry ID: {}, Ref ID: {}, Time: {}, Type: {}, Subtype: {}, Asset Class: {}, Asset: {}, Amount: {}, Fee: {}, Balance: {}",
                    entry_id,
                    ledger_entry.refid,
                    ledger_entry.time,
                    ledger_entry.ledger_type,
                    ledger_entry.subtype,
                    ledger_entry.aclass,
                    ledger_entry.asset,
                    ledger_entry.amount,
                    ledger_entry.fee,
                    ledger_entry.balance
                );
            }
            println!("Total count of ledger entries retrieved: {}", response.count);
            Ok(())
        }
        Err(e) => {
            let error_message = format!("Failed to retrieve ledger entries: {}", e);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}

pub async fn get_stakeable_assets() -> Result<(), String> {
    println!("Fetching stakeable assets...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let request = client.get_stakeable_assets();

    match request.send().await {
        Ok(stakeable_assets) => {
            println!("Stakeable assets retrieved successfully:");
            for asset in stakeable_assets {
                println!("Asset: {}", asset.asset);
                println!("  Staking Asset: {}", asset.staking_asset);
                println!("  Method: {:?}", asset.method);
                println!("  On Chain: {:?}", asset.on_chain);
                println!("  Minimum Amount:");
                println!("    Unstaking: {:?}", asset.minimum_amount.as_ref().map(|a| &a.unstaking));
                println!("    Staking: {:?}", asset.minimum_amount.as_ref().map(|a| &a.staking));
                println!("  Enabled for User: {:?}", asset.enabled_for_user);
                println!();
            }
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error retrieving stakeable assets: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}
pub async fn get_order_book(pair: &str, count: u32) -> Result<(), String> {
    println!("Fetching order book...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let request = client.get_order_book(pair).count(count);

    match request.send().await {
        Ok(order_book) => {
            if let Some(book) = order_book.get(pair) {
                println!("Order book retrieved successfully for pair: {}", pair);
                println!("Asks:");
                for ask in &book.asks {
                    println!("Price: {}, Volume: {}, Timestamp: {}", ask.0, ask.1, ask.2);
                }
                println!("Bids:");
                for bid in &book.bids {
                    println!("Price: {}, Volume: {}, Timestamp: {}", bid.0, bid.1, bid.2);
                }
            } else {
                println!("No data found for pair: {}", pair);
            }
            Ok(())
        },
        Err(error) => {
            let error_message = format!("Error retrieving order book: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}



pub async fn get_tickers(pairs: &str) -> Result<(), String> {
    println!("Fetching tickers...");

    let client = RestClient::default();
    let request = client.get_tickers(pairs);

    match request.send().await {
        Ok(tickers) => {
            println!("Tickers retrieved successfully:");
            for (pair, ticker) in tickers {
                println!("Pair: {}, Ticker: {:?}", pair, ticker);
            }
            Ok(())
        },
        Err(error) => {
            let error_message = format!("Error retrieving tickers: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}

// pub struct CancelOrderBatchResponse {
//    pub count: u32,
//    pub results: HashMap<String, CancelOrderBatchResult>,
// }

pub async fn cancel_order_batch(txids: &str) -> Result<(), String> {
    println!("Canceling orders in batch...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let txids_vec: Vec<String> = txids.split(',').map(|s| s.to_string()).collect();
    let request = client.cancel_order_batch(txids_vec);

    match request.send().await {
        Ok(batch_result) => {
            println!("Orders canceled in batch successfully.");
            println!("Count: {}", batch_result.count);
            Ok(())
        },
        Err(error) => {
            let error_message = format!("Error canceling orders in batch: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}

async fn verify_order_helper(client: &RestClient, pair: &str, side: OrderSide, order_type: OrderType, price: &str, volume: &str) -> Result<(), String> {
    let request = client
        .add_order(pair, side, order_type, volume)
        .price(price)
        .validate(true); // Validate the order without placing it

    match request.send().await {
        Ok(_) => Ok(()),
        Err(error) => {
            if error.to_string().contains("EOrder:Insufficient funds") {
                Err("Insufficient funds to place the order.".to_string())
            } else {
                Err(format!("Error verifying order: {:?}", error))
            }
        },
    }
}
        //"market": Market order
        //"limit": Limit order
        //"stop-loss": Stop-loss order
        //"take-profit": Take-profit order
        //"stop-loss-limit": Stop-loss limit order
        //"take-profit-limit": Take-profit limit order
        //"settle-position": Settle position order

        pub async fn add_order(pair: &str, side: &str, order_type: &str, price: &str, volume: &str) -> Result<(), String> {
            println!("Adding order...");
        
            let (api_key, api_secret) = load_api_credentials();
            let client = RestClient::new(api_key, api_secret);
        
            let side_enum = match side {
                "buy" => OrderSide::Buy,
                "sell" => OrderSide::Sell,
                _ => return Err("Invalid order side. Expected 'buy' or 'sell'.".to_string()),
            };
        
            let order_type_enum = match order_type {
                "limit" => OrderType::Limit,
                "market" => OrderType::Market,
                // Add more order types as needed
                _ => return Err("Invalid order type.".to_string()),
            };
        
            // Verify the order
            match verify_order_helper(&client, pair, side_enum, order_type_enum, price, volume).await {
                Ok(_) => {
                    // Place the order
                    let request = client
                        .add_order(pair, side_enum, order_type_enum, volume)
                        .price(price)
                        .userref(12345);
        
                    match request.send().await {
                        Ok(order_result) => {
                            println!("Order added successfully:");
                            println!("Description: {:?}", order_result.descr);
        
                            if let Some(txids) = order_result.txid {
                                println!("Transaction IDs:");
                                for txid in txids {
                                    println!("- {}", txid);
                                }
                            } else {
                                println!("Order not executed. Check your account balance and order details.");
                            }
                            Ok(())
                        }
                        Err(error) => {
                            let error_message = format!("Error adding order: {:?}", error);
                            eprintln!("{}", error_message);
                            Err(error_message)
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Order verification failed: {}", error);
                    Err(error)
                }
            }
        }

pub async fn verify_order(pair: &str, side: &str, order_type: &str, price: &str, volume: &str) -> Result<(), String> {
    // change this to check balance or check the API further
    println!("Verifying order parameters...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let side_enum = match side {
        "buy" => OrderSide::Buy,
        "sell" => OrderSide::Sell,
        _ => return Err("Invalid order side. Expected 'buy' or 'sell'.".to_string()),
    };

    let order_type_enum = match order_type {
        "limit" => OrderType::Limit,
        "market" => OrderType::Market,
        // Add more order types as needed
        _ => return Err("Invalid order type.".to_string()),
    };

    let request = client
        .add_order(pair, side_enum, order_type_enum, volume)
        .price(price)
        .validate(true); // Validate the order parameters without placing the order

    match request.send().await {
        Ok(order_result) => {
            println!("Order parameters verified successfully:");
            println!("Description: {:?}", order_result.descr);
            println!("Note: This does not guarantee sufficient funds for the order.");
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error verifying order parameters: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}


pub async fn edit_order() {
    println!("i dont think there is an edit order. oh well. Editing order...");

 /*   let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let txid = "OQCLML-7WKL3-PBVWWP"; // Example order ID to edit
    let price = "29000"; // New price
    let volume = "0.00002"; // New volume

    let request = client
        .edit_order(txid)
        .price(price)
        .volume(volume);

    match request.send().await {
        Ok(edit_result) => {
            println!("Order edited successfully:");
            println!("Description: {:?}", edit_result.descr);
        },
        Err(error) => {
            eprintln!("Error editing order: {:?}", error);
        }
    }*/ 
}

/// Cancels all open orders on the Kraken API.
pub async fn cancel_all_orders() -> Result<(), String> {
    println!("Attempting to cancel all open orders...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.cancel_all_orders();

    match request.send().await {
        Ok(response) => {
            println!("Successfully cancelled orders. Total cancelled orders: {}", response.count);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to cancel all orders: {}", e);
            Err(format!("Failed to cancel all orders: {}", e))
        }
    }
}


pub async fn cancel_order(txid: &str) -> Result<(), String> {
    println!("Canceling order...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);
    let request = client.cancel_order(txid);

    match request.send().await {
        Ok(canceled_order) => {
            println!("Order canceled successfully:");
            println!("Count: {}", canceled_order.count);
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error canceling order: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}


pub async fn get_closed_orders(trades: bool, start: Option<i64>, end: Option<i64>, ofs: Option<i64>) -> Result<(), String> {
    println!("Fetching closed orders...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let mut request = client.get_closed_orders().trades(trades);

    if let Some(start_time) = start {
        request = request.start(start_time);
    }
    if let Some(end_time) = end {
        request = request.end(end_time);
    }
    if let Some(offset) = ofs {
        request = request.ofs(offset.try_into().unwrap_or(0));
    }

    match request.send().await {
        Ok(closed_orders) => {
            println!("Closed orders retrieved successfully:");
            for (txid, order) in &closed_orders.closed {
                println!("Order ID: {}", txid);
                println!("  Status: {}", order.status);
                println!("  Reason: {:?}", order.reason);
                println!("  Open time: {}", order.opentm);
                println!("  Close time: {}", order.closetm);
                println!("  Expire time: {}", order.expiretm);
                println!("  Volume: {}", order.vol);
                println!("  Executed volume: {}", order.vol_exec);
                println!("  Fee: {}", order.fee);
                println!("  Limit price: {}", order.limitprice);
                println!("  Misc: {}", order.misc);
                println!();
            }
            println!("Count: {}", closed_orders.count);
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error retrieving closed orders: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}

pub async fn get_assets(assets: &str) -> Result<(), String> {
    println!("Fetching assets...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let request = client.get_assets().asset(assets);

    match request.send().await {
        Ok(asset_info) => {
            println!("Asset information retrieved successfully:");
            for (asset, info) in asset_info {
                println!("Asset: {}", asset);
                println!("  Altname: {}", info.altname);
                println!("  Asset class: {}", info.aclass);
                println!("  Decimals: {}", info.decimals);
                println!("  Display decimals: {}", info.display_decimals);
                println!();
            }
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error retrieving asset information: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
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


pub async fn get_asset_pairs() -> Result<(), String> {
    println!("Fetching asset pairs...");

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
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error retrieving asset pairs: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}

// pub struct DepositMethods {
//     pub method: String,
//     pub fee: Option<String>,
//     pub address_setup_fee: Option<String>,
//     pub gen_address: Option<bool>,
// }

pub async fn get_deposit_methods(asset: &str) -> Result<(), String> {
    println!("Fetching Deposit Methods...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let request = client.get_deposit_methods(asset);

    match request.send().await {
        Ok(deposit_methods) => {
            println!("Deposit methods retrieved successfully:");
            for method in deposit_methods {
                println!("Method: {}", method.method);
                println!("  Fee: {:?}", method.fee);
                println!("  Address setup fee: {:?}", method.address_setup_fee);
                println!("  Generate new address: {:?}", method.gen_address);
                println!();
            }
            Ok(())
        }
        Err(error) => {
            let error_message = format!("Error retrieving deposit methods: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}

pub async fn get_deposit_status(asset: &str, method: Option<&str>) -> Result<(), String> {
    println!("Fetching deposit status...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let mut request = client.get_deposit_status(asset);

    if let Some(m) = method {
        request = request.method(m.to_string());
    }

    match request.send().await {
        Ok(deposit_statuses) => {
            println!("Deposit statuses retrieved successfully:");
            for status in deposit_statuses {
                println!("Method: {}, Asset: {}, Reference ID: {}, TX ID: {}, Amount: {}, Time: {}, Status: {}",
                         status.method, status.asset, status.refid, status.txid, status.amount, status.time, status.status);
            }
            Ok(())
        },
        Err(error) => {
            let error_message = format!("Error retrieving deposit status: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}

pub async fn get_deposit_addresses(asset: &str, method: &str) -> Result<(), String> {
    println!("Fetching deposit addresses...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    let request = client.get_deposit_addresses(asset, method);

    match request.send().await {
        Ok(deposit_addresses) => {
            println!("Deposit addresses retrieved successfully:");
            for address in deposit_addresses {
                println!("Address: {}, Expire Time: {}, New: {:?}",
                         address.address, address.expiretm, address.new);
            }
            Ok(())
        },
        Err(error) => {
            let error_message = format!("Error retrieving deposit addresses: {:?}", error);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}
