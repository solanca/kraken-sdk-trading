# README.ai for kraken-sdk

# Project Overview
- Name: kraken-sdk
- Description: 
A Rust SDK for working with Kraken Exchange APIs.

This project is an unofficial, community-driven effort.

Components
The SDK contains the following components:

kraken_rest_client
kraken_ws_client
kraken_futures_rest_client
kraken_futures_ws_client
.
- Main Language: rust
- Dependencies: 

# Variable and State Information
- Point a
- Point b
- Point c

# Repository Structure
```
.
├── ./.github
│   └── ./.github/FUNDING.yml
├── ./.gitignore
├── ./Cargo.toml
├── ./LICENSE.txt
├── ./README.md
├── ./kraken_cli
│   ├── ./kraken_cli/Cargo.toml
│   ├── ./kraken_cli/LICENSE.txt
│   ├── ./kraken_cli/README.md
│   └── ./kraken_cli/src
│       └── ./kraken_cli/src/main.rs
├── ./kraken_futures_rest_client
│   ├── ./kraken_futures_rest_client/Cargo.toml
│   ├── ./kraken_futures_rest_client/LICENSE.txt
│   ├── ./kraken_futures_rest_client/README.md
│   └── ./kraken_futures_rest_client/src
│       ├── ./kraken_futures_rest_client/src/api
│       │   ├── ./kraken_futures_rest_client/src/api/get_instruments.rs
│       │   └── ./kraken_futures_rest_client/src/api/get_ohlc.rs
│       ├── ./kraken_futures_rest_client/src/api.rs
│       ├── ./kraken_futures_rest_client/src/client.rs
│       ├── ./kraken_futures_rest_client/src/error.rs
│       └── ./kraken_futures_rest_client/src/lib.rs
├── ./kraken_futures_ws_client
│   ├── ./kraken_futures_ws_client/Cargo.toml
│   ├── ./kraken_futures_ws_client/LICENSE.txt
│   ├── ./kraken_futures_ws_client/README.md
│   └── ./kraken_futures_ws_client/src
│       └── ./kraken_futures_ws_client/src/lib.rs
├── ./kraken_rest_client
│   ├── ./kraken_rest_client/Cargo.toml
│   ├── ./kraken_rest_client/LICENSE.txt
│   ├── ./kraken_rest_client/README.md
│   └── ./kraken_rest_client/src
│       ├── ./kraken_rest_client/src/api
│       │   ├── ./kraken_rest_client/src/api/add_order.rs
│       │   ├── ./kraken_rest_client/src/api/cancel_all_orders.rs
│       │   ├── ./kraken_rest_client/src/api/cancel_order.rs
│       │   ├── ./kraken_rest_client/src/api/cancel_order_batch.rs
│       │   ├── ./kraken_rest_client/src/api/get_account_balance.rs
│       │   ├── ./kraken_rest_client/src/api/get_asset_pairs.rs
│       │   ├── ./kraken_rest_client/src/api/get_assets.rs
│       │   ├── ./kraken_rest_client/src/api/get_closed_orders.rs
│       │   ├── ./kraken_rest_client/src/api/get_deposit_addresses.rs
│       │   ├── ./kraken_rest_client/src/api/get_deposit_methods.rs
│       │   ├── ./kraken_rest_client/src/api/get_deposit_status.rs
│       │   ├── ./kraken_rest_client/src/api/get_ledgers.rs
│       │   ├── ./kraken_rest_client/src/api/get_ohlc_data.rs
│       │   ├── ./kraken_rest_client/src/api/get_open_orders.rs
│       │   ├── ./kraken_rest_client/src/api/get_open_positions.rs
│       │   ├── ./kraken_rest_client/src/api/get_order_book.rs
│       │   ├── ./kraken_rest_client/src/api/get_recent_trades.rs
│       │   ├── ./kraken_rest_client/src/api/get_server_time.rs
│       │   ├── ./kraken_rest_client/src/api/get_stakeable_assets.rs
│       │   ├── ./kraken_rest_client/src/api/get_system_status.rs
│       │   ├── ./kraken_rest_client/src/api/get_tickers.rs
│       │   ├── ./kraken_rest_client/src/api/get_trade_balance.rs
│       │   ├── ./kraken_rest_client/src/api/get_trade_volume.rs
│       │   ├── ./kraken_rest_client/src/api/get_trades_history.rs
│       │   ├── ./kraken_rest_client/src/api/get_web_sockets_token.rs
│       │   ├── ./kraken_rest_client/src/api/get_withdrawal_addresses.rs
│       │   ├── ./kraken_rest_client/src/api/get_withdrawal_methods.rs
│       │   ├── ./kraken_rest_client/src/api/query_orders_info.rs
│       │   ├── ./kraken_rest_client/src/api/stake_asset.rs
│       │   ├── ./kraken_rest_client/src/api/unstake_asset.rs
│       │   └── ./kraken_rest_client/src/api/withdraw.rs
│       ├── ./kraken_rest_client/src/api.rs
│       ├── ./kraken_rest_client/src/client.rs
│       ├── ./kraken_rest_client/src/error.rs
│       ├── ./kraken_rest_client/src/lib.rs
│       ├── ./kraken_rest_client/src/sign.rs
│       ├── ./kraken_rest_client/src/types
│       │   ├── ./kraken_rest_client/src/types/asset_name.rs
│       │   ├── ./kraken_rest_client/src/types/order.rs
│       │   └── ./kraken_rest_client/src/types/pair_name.rs
│       └── ./kraken_rest_client/src/types.rs
└── ./kraken_ws_client
    ├── ./kraken_ws_client/Cargo.toml
    ├── ./kraken_ws_client/LICENSE.txt
    ├── ./kraken_ws_client/README.md
    ├── ./kraken_ws_client/examples
    │   ├── ./kraken_ws_client/examples/book.rs
    │   └── ./kraken_ws_client/examples/ticker.rs
    └── ./kraken_ws_client/src
        ├── ./kraken_ws_client/src/api
        │   ├── ./kraken_ws_client/src/api/add_order.rs
        │   ├── ./kraken_ws_client/src/api/batch_add.rs
        │   ├── ./kraken_ws_client/src/api/batch_cancel.rs
        │   ├── ./kraken_ws_client/src/api/cancel_all_orders.rs
        │   ├── ./kraken_ws_client/src/api/cancel_all_orders_after.rs
        │   ├── ./kraken_ws_client/src/api/cancel_order.rs
        │   ├── ./kraken_ws_client/src/api/edit_order.rs
        │   ├── ./kraken_ws_client/src/api/heartbeat.rs
        │   ├── ./kraken_ws_client/src/api/status.rs
        │   ├── ./kraken_ws_client/src/api/subscribe_book.rs
        │   ├── ./kraken_ws_client/src/api/subscribe_executions.rs
        │   ├── ./kraken_ws_client/src/api/subscribe_instrument.rs
        │   ├── ./kraken_ws_client/src/api/subscribe_ohlc.rs
        │   ├── ./kraken_ws_client/src/api/subscribe_ticker.rs
        │   └── ./kraken_ws_client/src/api/subscribe_trade.rs
        ├── ./kraken_ws_client/src/api.rs
        ├── ./kraken_ws_client/src/client.rs
        ├── ./kraken_ws_client/src/error.rs
        ├── ./kraken_ws_client/src/lib.rs
        ├── ./kraken_ws_client/src/types.rs
        └── ./kraken_ws_client/src/util.rs

16 directories, 91 files
```
- Point a
- Point b
- Point c

# Contribution Guidelines
- How to set up the development environment.
- Guidelines for submitting pull requests.
- Contact information for the project maintainers.

# Additional Notes
- Any other relevant information that an AI or a human might find useful.
