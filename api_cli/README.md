# Financial Exchange Command Line Interface  

This command-line interface (CLI) is designed to interact with financial exchange APIs. Currently, it supports only the Kraken exchange, with plans to abstract the interface to support additional exchanges in the future.  

## Getting Started  To get started with this CLI, clone the repository and navigate to the project directory:  

```bash git clone <repository-url> cd <project-directory>`

Usage
-----

To use the CLI, run the following command in your terminal:

bash

Copy code

`api_cli kraken <function_name>`

Where `<function_name>` is the name of the API function you want to execute. Here are a few examples of how to use the CLI with the Kraken exchange:

*   Retrieve your account balance:
    
    bash
    
    Copy code
    
    `api_cli kraken get_account_balance`
    
*   Get trade volume for a specific trading pair:
    
    bash
    
    Copy code
    
    `api_cli kraken get_trade_volume --pair XBTUSD`
    
*   Withdraw funds to a specified address:
    
    bash
    
    Copy code

    `api_cli kraken query_orders_info --txids OGCOLF-XCKCI-TK2YRI --trades true`
    

    `api_cli kraken add_order --pair solusd --side buy --type limit --price 50 --volume 45.2`

Supported Commands
------------------

The CLI currently supports the following functions for the Kraken exchange, including their options:
Certainly! Here's the complete listing of the supported commands for your Kraken CLI tool in a single code block:

```markdown
## Supported Commands

The CLI currently supports the following functions for the Kraken exchange, including detailed options for each command:

- **get_account_balance**: Retrieves the account balance.

- **get_trade_balance**: Retrieves the trade balance.

- **get_trade_history**: Retrieves trade history.

- **get_web_sockets_token**: Retrieves web sockets token.

- **get_withdrawal_addresses**: Retrieves withdrawal addresses.
  - `asset`: The asset to filter by (e.g., XBT)
  - `aclass`: The asset class to filter by (e.g., currency)
  - `method`: The withdrawal method to filter by (e.g., Bitcoin)

- **get_withdrawal_status**: Checks the status of a withdrawal.

- **unstake_asset**: Unstakes an asset.

- **get_trade_volume**: Retrieves trade volume.
  - `pair`: The trading pair (e.g., XBTUSD)

- **query_orders_info**: Queries orders information.
  - `txids`: Comma-separated list of transaction IDs
  - `trades`: Whether to include trade details (true or false)

- **stake_asset**: Stakes an asset.
  - `asset`: The asset to stake (e.g., XBT)
  - `amount`: The amount to stake (e.g., 0.0001)
  - `method`: The staking method (e.g., XBT.M)

- **withdraw**: Processes a withdrawal.
  - `asset`: The asset to withdraw (e.g., XBT)
  - `key`: The withdrawal key (e.g., my_withdrawal_key)
  - `amount`: The amount to withdraw (e.g., 0.1)

- **get_withdrawal_methods**: Retrieves withdrawal methods.
  - `asset`: The asset to filter by (e.g., XBT)

- **get_open_positions**: Retrieves open positions.

- **get_system_status**: Checks the system status.

- **get_server_time**: Retrieves the server time.

- **get_open_orders**: Retrieves open orders.
  - `trades`: Whether to include trade details (true or false)

- **get_recent_trades**: Retrieves recent trades.
  - `pair`: The trading pair (e.g., BTC/USD)
  - `since`: Start timestamp (Unix timestamp)
  - `count`: Maximum number of trades to retrieve per request

- **get_public_ohlc_data**: Retrieves the last 720 candles of public OHLC data.
  - `base`: The base currency of the trading pair (e.g., XBT)
  - `quote`: The quote currency of the trading pair (e.g., USD)
  - `interval`: The time interval in minutes (e.g., 1, 5, 15, etc.)

- **get_ledgers**: Retrieves ledger entries.
  - `aclass`: The asset class to filter by (e.g., currency)

- **get_stakeable_assets**: Retrieves stakeable assets.

- **get_order_book**: Retrieves the order book.
  - `pair`: The trading pair (e.g., XXBTZUSD)
  - `count`: The maximum number of asks/bids to retrieve

- **get_tickers**: Retrieves tickers.
  - `pairs`: Comma-separated list of trading pairs (e.g., XXBTZUSD,DOTUSD)

- **cancel_order_batch**: Cancels a batch of orders.
  - `txids`: Comma-separated list of transaction IDs to cancel

- **add_order**: Adds a new order.
  - `pair`: The trading pair (e.g., SOLGBP)
  - `side`: The order side (buy or sell)
  - `type`: The order type (limit, market, etc.)
  - `price`: The order price (e.g., 50)
  - `volume`: The order volume (e.g., 0.5)

- **edit_order**: Edits an existing order.
  - `txid`: The transaction ID of the order to edit
  - `price`: The new order price
  - `volume`: The new order volume

- **verify_order**: Verifies an order.
  - `pair`: The trading pair (e.g., SOLGBP)
  - `side`: The order side (buy or sell)
  - `type`: The order type (limit, market, etc.)
  - `price`: The order price (e.g., 50)
  - `volume`: The order volume (e.g., 0.5)

- **cancel_all_orders**: Cancels all orders.

- **cancel_order**: Cancels a specific order.
  - `txid`: The transaction ID of the order to cancel
  
- **get_closed_orders**: Retrieves closed orders.
  - `trades`: Whether to include trade details (true or false).
  - `userref`: Filter closed orders by user reference id.
  - `start`: Start time for the query in Unix timestamp.
  - `end`: End time for the query in Unix timestamp.
  - `ofs`: Result offset for pagination.
  - `closetime`: Filter by order close time (`open`, `close`, `both`).
```

## Additional Notes

For more detailed information on each command, including potential error messages and handling, please refer to the Kraken API documentation. This will provide you with a comprehensive understanding of how each function interacts with the Kraken platform and how to handle various responses from the API.

### Troubleshooting

If you encounter any issues while using this CLI, tough.

Thank you for using this CLI. Your feedback and suggestions are always not welcome to improve the tool and expand its capabilities.