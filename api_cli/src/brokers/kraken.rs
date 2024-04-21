use kraken_rest_client::{Client as RestClient};
use kraken_rest_client::api::get_ohlc_data::{Interval};
use kraken_rest_client::types::pair_name::PairName;

use std::env;
use dotenv::dotenv;

fn load_api_credentials() -> (String, String) {
    dotenv().ok();
    let api_key = env::var("KRAKEN_API_KEY").expect("KRAKEN_API_KEY not set");
    let api_secret = env::var("KRAKEN_API_SECRET").expect("KRAKEN_API_SECRET not set");
    (api_key, api_secret)
}


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

pub async fn get_account_balance() {
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
        }
        Err(error) => {
            eprintln!("Error retrieving trades history: {:?}", error);
        }
    }
}

//pub struct GetWebSocketsTokenResponse {
//    pub token: String,
//    pub expires: i64,
//}

/// Retrieves a WebSocket token from the Kraken API for authenticating WebSocket connections.
pub async fn get_web_sockets_token() {
    println!("Retrieving WebSocket token...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_web_sockets_token();

    match request.send().await {
        Ok(response) => {
            println!("WebSocket token retrieved successfully:");
            println!("Token: {}, Expires in: {} seconds", response.token, response.expires);
        }
        Err(e) => {
            eprintln!("Failed to retrieve WebSocket token: {}", e);
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

pub async fn get_trade_volume() {
    println!("Testing get_trade_volume...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let pair = "XBTUSD";  // Hardcoded pair, adjust as necessary
    let request = client.get_trade_volume(pair);  // Correctly supplying the required parameter

    match request.send().await {
        Ok(trade_volume) => {
            println!("Trade volume retrieved successfully:");
            println!("Currency: {}", trade_volume.currency);
            println!("Volume: {}", trade_volume.volume);
            println!("Fees: {:?}", trade_volume.fees);  // Use debug formatting for HashMap
            println!("Maker Fees: {:?}", trade_volume.fees_maker);  // Use debug formatting for HashMap
        }
        Err(error) => {
            eprintln!("Error retrieving trade volume: {:?}", error);
        }
    }
}





pub async fn query_orders_info() {
    println!("Testing query_orders_info...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);

    // Replace with the actual transaction IDs you want to query
    let txids = "OWJ5IB-7VTBA-ATKFS4,OHN4FQ-TJR6U-TG5HNV";

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

/// Retrieves the open positions from the Kraken API, always including P&L calculations.
pub async fn get_open_positions() {
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
        }
        Err(e) => {
            eprintln!("Failed to retrieve open positions: {}", e);
        }
    }
}


pub async fn get_system_status() {
    println!("Testing get_system_status...");


    let (api_key, api_secret) = load_api_credentials();

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


    let (api_key, api_secret) = load_api_credentials();

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


/// Retrieves the open orders from the Kraken API.
pub async fn get_open_orders() {
    println!("Retrieving open orders...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_open_orders().trades(true); // Assuming you might want trade details by default

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
                    order_info.descr.price2, // Correct field names used
                    order_info.opentm,
                    order_info.oflags,
                    order_info.fee,
                    order_info.vol,
                    order_info.vol_executed.unwrap_or("0".to_string()),
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to retrieve open orders: {}", e);
        }
    }
}




/// Retrieves the recent trades for the hardcoded pair "BTC/USD" from the Kraken API.
pub async fn get_recent_trades() {
    println!("Retrieving recent trades for pair: BTC/USD");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_recent_trades("BTC/USD");

    match request.send().await {
        Ok(response) => {
            println!("Recent trades retrieved successfully:");
            for (pair_key, trades) in response.pair {
                println!("Trading pair: {}", pair_key);
                for trade in trades {
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
                }
            }
            println!("Last ID for further requests: {}", response.last);
        }
        Err(e) => {
            eprintln!("Failed to retrieve recent trades: {}", e);
        }
    }
}


/// Retrieves ledger entries from the Kraken API, hardcoding for currency asset class and no specific filtering.
pub async fn get_ledgers() {
    println!("Retrieving ledger entries...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.get_ledgers().aclass("currency");  // Hardcoded to fetch all currency class assets

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
        }
        Err(e) => {
            eprintln!("Failed to retrieve ledger entries: {}", e);
        }
    }
}


pub async fn get_stakeable_assets() {
    println!("Testing get_stakeable_assets...");
}

pub async fn get_order_book() {
    println!("Fetching order book...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);
  // Example: Get the order book for the "XBT/USD" pair with a maximum of 100 entries
  let pair = "XXBTZUSD";
  let count = 100; // Maximum number of asks/bids

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
      }
      Err(error) => {
          eprintln!("Error retrieving order book: {:?}", error);
      }
  }
}



pub async fn get_tickers() {
    println!("Testing get_tickers...");

    let client = RestClient::default(); // Use the default REST client
    let request = client.get_tickers("XXBTZUSD,DOTUSD"); // Specify the asset pairs to retrieve tickers for here    

    match request.send().await {
        Ok(tickers) => {
            println!("Tickers retrieved successfully:");
            for (pair, ticker) in tickers {
                println!("Pair: {}, Ticker: {:?}", pair, ticker);
            }
        }
        Err(error) => {
            eprintln!("Error retrieving tickers: {:?}", error);
        }
    }   
}

pub async fn cancel_order_batch() {
    println!("Testing cancel_order_batch...");
}

pub async fn add_order() {
    println!("Testing add_order...");
}

/// Cancels all open orders on the Kraken API.
pub async fn cancel_all_orders() {
    println!("Attempting to cancel all open orders...");

    let (api_key, api_secret) = load_api_credentials();

    let client = RestClient::new(api_key, api_secret);
    let request = client.cancel_all_orders();

    match request.send().await {
        Ok(response) => {
            println!("Successfully cancelled orders. Total cancelled orders: {}", response.count);
        }
        Err(e) => {
            eprintln!("Failed to cancel all orders: {}", e);
        }
    }
}


pub async fn cancel_order() {
    println!("Testing cancel_order...");
}

pub async fn get_closed_orders() {
    println!("Testing get_closed_orders...");
}

pub async fn get_assets() {
    println!("Fetching assets...");

    let (api_key, api_secret) = load_api_credentials();
    let client = RestClient::new(api_key, api_secret);

    // Example: Get information about specific assets
    let assets = "XBT,ETH,USDT"; // Comma-separated list of assets

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
        }
        Err(error) => {
            eprintln!("Error retrieving asset information: {:?}", error);
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