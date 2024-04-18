Here's an enhanced version of your README file, formatted in Markdown, which now includes detailed descriptions of some key features provided by your Rust SDK for the Kraken REST API. This enhancement aims to give users a clearer understanding of what the library offers and how to utilize its capabilities effectively.

* * *

Kraken REST API Client
======================

A strongly-typed Rust SDK designed to facilitate easy and efficient interaction with the Kraken cryptocurrency exchange API.

Features
--------

*   **Strong Typing**: Ensures robust code and minimizes common programming errors by leveraging Rust's type system.
*   **Asynchronous Design**: Built for asynchronous execution to enhance performance, particularly useful in handling IO-bound tasks.
*   **Comprehensive Endpoint Coverage**: Provides structured access to both public and private API endpoints, covering a wide range of functionalities such as retrieving server time, market data, and managing orders.

Usage
-----

Here's a quick overview of how to perform various operations using the Kraken REST API Client:

rust

Copy code

`use kraken_rest_client::{Client, PairName, OrderSide, Interval};  let client = Client::new(     "YOUR-API-KEY",     "YOUR_API-SECRET", );  // Retrieve server time let resp = client.get_server_time().send().await?; println!("{}", resp.unixtime);  // Get OHLC data let pair = PairName::from("BTC", "USD"); let req = client.get_ohlc_data(&pair).interval(Interval::Day1); let resp = req.send().await; println!("{:?}", resp);  // Place a limit order let pair = "XXRPZUSD"; let resp = client     .add_limit_order(pair, OrderSide::Buy, "20", "0.10")     .expire_after(60 * 60)  // Set expiration time to 1 hour     .userref(123)           // Set a user reference id     .validate_only()        // Validate the order but do not execute     .send()     .await?; println!("{:?}", resp);  // Cancel an order let resp = client.cancel_order("O6CIT1-NABRS-TMVZ1X").send().await?; println!("{}", resp.count);`

API Endpoint Examples
---------------------

### Query Order Information

Retrieve detailed information about specific orders, optionally including associated trades, utilizing the robust querying capabilities of the SDK.

rust

Copy code

`let orders = client     .query_orders_info("OXEHQQ-R25RV-NDOGKM,ORQC1X-TSQHB-KIEE7I")     .trades(true)  // Include trades related to the orders     .send()     .await?;`

### Manage Orders

Handle orders programmatically, allowing operations such as placing, modifying, and cancelling orders with ease.

rust

Copy code

`let order_request = client.add_market_order("XXBTZUSD", OrderSide::Sell, "1.5")     .validate_only()     .send()     .await?;`

FAQ
---

### Why provide both `execute` and `send` methods for API endpoint handlers?

Providing the lower-level `execute` method allows for more flexibility. Since `execute` is generic, you can pass any type of object to deserialize the response to, e.g., you could deserialize to a `HashMap` instead of the 'default' response for each API call. Or you could use a custom struct with only the fields you are interested in. This design enables fine-grained control over response handling, catering to advanced use cases where standard deserialization is insufficient.

Certainly! Here's an expanded version of your README file formatted in Markdown, which now includes precise details on the main structs and their functionalities. This version aims to provide a more technical overview, especially beneficial for developers needing to integrate deeply with the API.


Key Structures
--------------

### `QueryOrdersInfoRequest`

Responsible for constructing and sending requests to query detailed information about specific orders identified by their transaction IDs.

*   **Attributes**:
    *   `trades`: Option to include trades related to the position in the output.
    *   `userref`: Restricts results to a given user reference ID.
*   **Method**:
    *   `execute()`: Builds the query string from the instance attributes and sends the request using `Client::send_private()`, returning the response asynchronously.

rust

Copy code

`pub struct QueryOrdersInfoRequest {     client: Client,     trades: Option<bool>,     userref: Option<i32>,     txid: String, }`

### `OrderInfo`

Describes the structure of the response from an order query. This struct includes detailed attributes of an order.

*   **Fields**:
    *   `status`: The status of the order.
    *   `vol`: Volume of the order.
    *   `vol_exec`: Executed volume.
    *   `cost`: Total cost.
    *   `fee`: Associated fees.
    *   `opentm`: Opening time.
    *   `expiretm`: Expiry time.

rust

Copy code

`#[derive(Debug, Deserialize)] pub struct OrderInfo {     userref: Option<i32>,     status: String,     vol: String,     vol_exec: String,     cost: String,     fee: String,     opentm: f64,     expiretm: f64, }`

### `QueryOrdersInfoResponse`

A type alias for a `HashMap` where each key-value pair corresponds to a transaction ID and its associated `OrderInfo`.

rust

Copy code

`pub type QueryOrdersInfoResponse = HashMap<String, OrderInfo>;`

### Client Extensions

Methods added to the `Client` struct to facilitate creating and managing `QueryOrdersInfoRequest` instances.

rust

Copy code

`impl Client {     pub fn query_orders_info(&self, txid: &str) -> QueryOrdersInfoRequest {         QueryOrdersInfoRequest {             client: self.clone(),             trades: None,             userref: None,             txid: txid.to_owned(),         }     } }`

Practical Usage
---------------

### Querying Order Information

Retrieve detailed information about specific orders with optional trades inclusion:

rust

Copy code

`let orders = client     .query_orders_info("OXEHQQ-R25RV-NDOGKM,ORQC1X-TSQHB-KIEE7I")     .trades(true)     .send()     .await?;`



* * *


