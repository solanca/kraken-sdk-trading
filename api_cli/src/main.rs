use std::env;
use std::fs;
use std::process;

mod brokers {
    pub mod kraken;
    pub mod coinbase;
    pub mod kucoin;
    pub mod binance;
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <broker> <function_name>", args[0]);
        eprintln!("Available brokers:");
        print_available_brokers();
        process::exit(1);
    }

    let broker = &args[1];
    let function_name = &args[2];

    match broker.as_str() {
        "kraken" => brokers::kraken::handle_function(function_name).await,
        "coinbase" => brokers::coinbase::handle_function(function_name).await,
        "kucoin" => brokers::kucoin::handle_function(function_name).await,
        "binance" => brokers::binance::handle_function(function_name).await,
        _ => {
            eprintln!("Broker '{}' not recognized.", broker);
            process::exit(1);
        }
    }
}

fn print_available_brokers() {
    let brokers_dir = fs::read_dir("src/brokers").unwrap();
    for entry in brokers_dir {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".rs") && file_name != "mod.rs" {
                    let broker_name = &file_name[..file_name.len() - 3];
                    eprintln!("  {}", broker_name);
                }
            }
        }
    }
}