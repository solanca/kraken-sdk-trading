mod tests;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <function_name>", args[0]);
        eprintln!("List of available functions:");
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
        std::process::exit(1);
    }

    let function_name = &args[1];
    match function_name.as_str() {
        "get_account_balance" => tests::get_account_balance().await,
        "get_trade_balance" => tests::get_trade_balance().await,
        "get_trades_history" => tests::get_trades_history().await,
        "get_web_sockets_token" => tests::get_web_sockets_token().await,
        "get_withdrawal_addresses" => tests::get_withdrawal_addresses().await,
        "unstake_asset" => tests::unstake_asset().await,
        "get_trade_volume" => tests::get_trade_volume().await,
        "query_orders_info" => tests::query_orders_info().await,
        "stake_asset" => tests::stake_asset().await,
        "withdraw" => tests::withdraw().await,
        "get_withdrawal_methods" => tests::get_withdrawal_methods().await,
        "get_open_positions" => tests::get_open_positions().await,
        "get_system_status" => tests::get_system_status().await,
        "get_server_time" => tests::get_server_time().await,
        "get_open_orders" => tests::get_open_orders().await,
        "get_recent_trades" => tests::get_recent_trades().await,
        "get_ohlc_data" => tests::get_ohlc_data().await,
        "get_ledgers" => tests::get_ledgers().await,
        "get_stakeable_assets" => tests::get_stakeable_assets().await,
        "get_order_book" => tests::get_order_book().await,
        "get_tickers" => tests::get_tickers().await,
        "cancel_order_batch" => tests::cancel_order_batch().await,
        "add_order" => tests::add_order().await,
        "cancel_all_orders" => tests::cancel_all_orders().await,
        "cancel_order" => tests::cancel_order().await,
        "get_closed_orders" => tests::get_closed_orders().await,
        "get_assets" => tests::get_assets().await,
        "get_asset_pairs" => tests::get_asset_pairs().await,
        "get_deposit_methods" => tests::get_deposit_methods().await,
        _ => {
            eprintln!("Function '{}' not recognized.", function_name);
            std::process::exit(1);
        }
    }
}
