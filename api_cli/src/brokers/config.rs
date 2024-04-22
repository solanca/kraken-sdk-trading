use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::env;

/// Loads configurations for all brokers and returns them in a HashMap.
pub fn load_configs() -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let brokers = vec!["binance", "coinbase", "kraken", "kucoin"]; // Adjust the list as necessary
    let mut configs = HashMap::new();
    let mut results = String::new();

       // Print the current working directory
       let current_dir = env::current_dir()?;
       println!("Current working directory: {:?}", current_dir);

    for broker in brokers {
        let file_name = format!("src/brokers/{}.json", broker); // Ensure the path is correct
        match fs::read_to_string(&file_name) {
            Ok(data) => {
                let json: Value = serde_json::from_str(&data)?;
                configs.insert(broker.to_string(), json);
                results.push_str(&format!("{} ✓ ", broker)); // Append success mark
            },
            Err(e) => {
                results.push_str(&format!("{} ✗ ", broker)); // Append failure mark
                eprintln!("Failed to load configuration for {}: {}", broker, e);
            }
        }
    }

    println!("Brokers loaded: {}", results.trim_end());
    Ok(configs)
}
