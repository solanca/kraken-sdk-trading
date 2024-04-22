# Cryptocurrency Exchange API CLI

This command-line interface (CLI) allows you to interact with various cryptocurrency exchange APIs, including Kraken, Coinbase, KuCoin, and Binance. It provides a convenient way to test and execute different API functions for each exchange.

## Usage

To use the CLI, simply run the following command:

```
api_cli <broker> <function_name>
```

- `<broker>`: The name of the cryptocurrency exchange (e.g., `kraken`, `coinbase`, `kucoin`, `binance`).
- `<function_name>`: The name of the API function you want to execute.

## Available Brokers

The CLI supports the following cryptocurrency exchanges:

once upon a time you had 4 brokers, but now you have 1 brokers. 

- Kraken - THIS ONE ONLY 
- Coinbase
- KuCoin
- Binance

To get started, you can use the `kraken` broker with the `get_account_balance` function.

```bash 
api_cli kraken get_account_balance
```

## Kraken Functions

The following functions are available for the Kraken exchange:

- `get_account_balance`: Test the account balance API.
- `get_trade_balance`: Test the trade balance API.
- `get_trades_history`: Test trades history API.
- `get_web_sockets_token`: Test WebSocket token retrieval.
- `get_withdrawal_addresses`: Test withdrawal addresses retrieval.
- `unstake_asset`: Test asset unstaking.
- `get_trade_volume`: Test trade volume retrieval.
- `query_orders_info`: Test orders information query.
- `stake_asset`: Test asset staking.
- `withdraw`: Test withdrawing funds.
- `get_withdrawal_methods`: Test withdrawal methods retrieval.
- `get_open_positions`: Test open positions retrieval.
- `get_system_status`: Test system status check.
- `get_server_time`: Test server time check.
- `get_open_orders`: Test open orders retrieval.
- `get_recent_trades`: Test recent trades retrieval.
- `get_ohlc_data`: Test OHLC data retrieval.
- `get_public_ohlc_data`: Test public OHLC data retrieval.
- `get_ledgers`: Test ledgers retrieval.
- `get_stakeable_assets`: Test stakeable assets retrieval.
- `get_order_book`: Test order book retrieval.
- `get_tickers`: Test tickers retrieval.
- `cancel_order_batch`: Test batch order cancellation.
- `add_order`: Test adding orders.
- `cancel_all_orders`: Test cancellation of all orders.
- `cancel_order`: Test single order cancellation.
- `get_closed_orders`: Test closed orders retrieval.
- `get_assets`: Test assets retrieval.
- `get_asset_pairs`: Test asset pairs retrieval.
- `get_deposit_methods`: Test deposit methods retrieval.

## Coinbase Functions

The following functions are available for the Coinbase exchange:

- `get_account_balance`: Test the account balance API.
- `get_trade_balance`: Test the trade balance API.
- `get_trades_history`: Test trades history API.
- `get_web_sockets_token`: Test WebSocket token retrieval.
- `get_withdrawal_addresses`: Test withdrawal addresses retrieval.
- `unstake_asset`: Test asset unstaking.
- `get_trade_volume`: Test trade volume retrieval.
- `query_orders_info`: Test orders information query.
- `stake_asset`: Test asset staking.
- `withdraw`: Test withdrawing funds.
- `get_withdrawal_methods`: Test withdrawal methods retrieval.
- `get_open_positions`: Test open positions retrieval.
- `get_system_status`: Test system status check.
- `get_server_time`: Test server time check.
- `get_open_orders`: Test open orders retrieval.
- `get_recent_trades`: Test recent trades retrieval.
- `get_ohlc_data`: Test OHLC data retrieval.
- `get_public_ohlc_data`: Test public OHLC data retrieval.
- `get_ledgers`: Test ledgers retrieval.
- `get_stakeable_assets`: Test stakeable assets retrieval.
- `get_order_book`: Test order book retrieval.
- `get_tickers`: Test tickers retrieval.
- `cancel_order_batch`: Test batch order cancellation.
- `add_order`: Test adding orders.
- `cancel_all_orders`: Test cancellation of all orders.
- `cancel_order`: Test single order cancellation.
- `get_closed_orders`: Test closed orders retrieval.
- `get_assets`: Test assets retrieval.
- `get_asset_pairs`: Test asset pairs retrieval.
- `get_deposit_methods`: Test deposit methods retrieval.

## KuCoin Functions

The following functions are available for the KuCoin exchange:

- `get_account_balance`: Test the account balance API.
- `get_trade_balance`: Test the trade balance API.
- `get_trades_history`: Test trades history API.
- `get_web_sockets_token`: Test WebSocket token retrieval.
- `get_withdrawal_addresses`: Test withdrawal addresses retrieval.
- `unstake_asset`: Test asset unstaking.
- `get_trade_volume`: Test trade volume retrieval.
- `query_orders_info`: Test orders information query.
- `stake_asset`: Test asset staking.
- `withdraw`: Test withdrawing funds.
- `get_withdrawal_methods`: Test withdrawal methods retrieval.
- `get_open_positions`: Test open positions retrieval.
- `get_system_status`: Test system status check.
- `get_server_time`: Test server time check.
- `get_open_orders`: Test open orders retrieval.
- `get_recent_trades`: Test recent trades retrieval.
- `get_ohlc_data`: Test OHLC data retrieval.
- `get_public_ohlc_data`: Test public OHLC data retrieval.
- `get_ledgers`: Test ledgers retrieval.
- `get_stakeable_assets`: Test stakeable assets retrieval.
- `get_order_book`: Test order book retrieval.
- `get_tickers`: Test tickers retrieval.
- `cancel_order_batch`: Test batch order cancellation.
- `add_order`: Test adding orders.
- `cancel_all_orders`: Test cancellation of all orders.
- `cancel_order`: Test single order cancellation.
- `get_closed_orders`: Test closed orders retrieval.
- `get_assets`: Test assets retrieval.
- `get_asset_pairs`: Test asset pairs retrieval.
- `get_deposit_methods`: Test deposit methods retrieval.

## Binance Functions

The following functions are available for the Binance exchange:

- `get_account_balance`: Test the account balance API.
- `get_trade_balance`: Test the trade balance API.
- `get_trades_history`: Test trades history API.
- `get_web_sockets_token`: Test WebSocket token retrieval.
- `get_withdrawal_addresses`: Test withdrawal addresses retrieval.
- `unstake_asset`: Test asset unstaking.
- `get_trade_volume`: Test trade volume retrieval.
- `query_orders_info`: Test orders information query.
- `stake_asset`: Test asset staking.
- `withdraw`: Test withdrawing funds.
- `get_withdrawal_methods`: Test withdrawal methods retrieval.
- `get_open_positions`: Test open positions retrieval.
- `get_system_status`: Test system status check.
- `get_server_time`: Test server time check.
- `get_open_orders`: Test open orders retrieval.
- `get_recent_trades`: Test recent trades retrieval.
- `get_ohlc_data`: Test OHLC data retrieval.
- `get_public_ohlc_data`: Test public OHLC data retrieval.
- `get_ledgers`: Test ledgers retrieval.
- `get_stakeable_assets`: Test stakeable assets retrieval.
- `get_order_book`: Test order book retrieval.
- `get_tickers`: Test tickers retrieval.
- `cancel_order_batch`: Test batch order cancellation.
- `add_order`: Test adding orders.
- `cancel_all_orders`: Test cancellation of all orders.
- `cancel_order`: Test single order cancellation.
- `get_closed_orders`: Test closed orders retrieval.
- `get_assets`: Test assets retrieval.
- `get_asset_pairs`: Test asset pairs retrieval.
- `get_deposit_methods`: Test deposit methods retrieval.

## Examples

Here are a few examples of how to use the CLI:

- Retrieve account balance for Kraken:
  ```
  api_cli kraken get_account_balance
  ```

- Get recent trades for Coinbase:
  ```
  api_cli coinbase get_recent_trades
  ```

- Cancel all orders on KuCoin:
  ```
  api_cli kucoin cancel_all_orders
  ```

- Get OHLC data from Binance:
  ```
  api_cli binance get_ohlc_data
  ```

Please note that some functions may require additional authentication or configuration depending on the exchange's API requirements.

## Contributing

If you would like to contribute to this project or add support for more exchanges and functions, please feel free to ask kraken, binance, kucoin and coinbase to give me money.

## Licence

The licence is not divulged and it is not open source until kraken pay.

## Copyright

© 100mountains 2024 and kraken-sdk by © 2023 [Georgios Moschovitis](https://gmosx.ninja).
