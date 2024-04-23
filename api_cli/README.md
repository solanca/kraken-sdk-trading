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
    
    `api_cli kraken withdraw --asset XBT --key my_withdrawal_key --amount 0.1`
    

Supported Commands
------------------

The CLI currently supports the following functions for the Kraken exchange, including their options:

*   **get\_account\_balance**: Retrieves the account balance.
*   **get\_trade\_balance**: Retrieves the trade balance.
*   **get\_trade\_history**: Retrieves trade history.
*   **get\_web\_sockets\_token**: Retrieves web sockets token.
*   **get\_withdrawal\_addresses**: Retrieves withdrawal addresses.
    *   `asset`: The asset to filter by (e.g., XBT)
    *   `aclass`: The asset class to filter by (e.g., currency)
    *   `method`: The withdrawal method to filter by (e.g., Bitcoin)
*   **get\_withdrawal\_status**: Checks the status of a withdrawal.
*   **unstake\_asset**: Unstakes an asset.
*   **get\_trade\_volume**: Retrieves trade volume.
    *   `pair`: The trading pair (e.g., XBTUSD)
*   **query\_orders\_info**: Queries orders information.
    *   `txids`: Comma-separated list of transaction IDs
    *   `trades`: Whether to include trade details (true or false)
*   **stake\_asset**: Stakes an asset.
    *   `asset`: The asset to stake (e.g., XBT)
    *   `amount`: The amount to stake (e.g., 0.0001)
    *   `method`: The staking method (e.g., XBT.M)
*   **withdraw**: Processes a withdrawal.
    *   `asset`: The asset to withdraw (e.g., XBT)
    *   `key`: The withdrawal key (e.g., my\_withdrawal\_key)
    *   `amount`: The amount to withdraw (e.g., 0.1)
*   **get\_withdrawal\_methods**: Retrieves withdrawal methods.
    *   `asset`: The asset to filter by (e.g., XBT)
*   **get\_open\_positions**: Retrieves open positions.
*   **get\_system\_status**: Checks the system status.
*   **get\_server\_time**: Retrieves the server time.
*   **get\_open\_orders**: Retrieves open orders.
    *   `trades`: Whether to include trade details (true or false)
*   **get\_recent\_trades**: Retrieves recent trades.
    *   `pair`: The trading pair (e.g., BTC/USD)
    *   `since`: Start timestamp (Unix timestamp)
    *   `count`: Maximum number of trades to retrieve per request
*   **get\_public\_ohlc\_data**: Retrieves the last 720 candles of public OHLC data.
    *   `base`: The base currency of the trading pair (e.g., XBT)
    *   `quote`: The quote currency of the trading pair (e.g., USD)
    *   `interval`: The time interval in minutes (e.g., 1, 5, 15, etc.)
*   **get\_ledgers**: Retrieves ledger entries.
    *   `aclass`: The asset class to filter by (e.g., currency)
*   **get\_stakeable\_assets**: Retrieves stakeable assets.
*   **get\_order\_book**: Retrieves the order book.
    *   `pair`: The trading pair (e.g., XXBTZUSD)
    *   `count`: The maximum number of asks/bids to retrieve
*   **get\_tickers**: Retrieves tickers.
    *   `pairs`: Comma-separated list of trading pairs (e.g., XXBTZUSD,DOTUSD)
*   **cancel\_order\_batch**: Cancels a batch of orders.
    *   `txids`: Comma-separated list of transaction IDs to cancel
*   **add\_order**: Adds a new order.
    *   `pair`: The trading pair (e.g