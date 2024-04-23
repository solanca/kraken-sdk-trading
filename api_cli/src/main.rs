mod brokers;
// connector to route to SDKs 

use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;

fn generate_usage(command: &Value) -> String {
    let args_usage: Vec<String> = command["arguments"].as_array().unwrap_or(&vec![])
        .iter()
        .map(|arg| {
            let name = arg["name"].as_str().unwrap();
            format!("--{} <{}>", name, name)
        })
        .collect();
    format!("Usage: {} {}", command["name"].as_str().unwrap(), args_usage.join(" "))
}

fn generate_help_for_broker(commands: &[Value]) -> String {
    commands.iter().map(generate_usage).collect::<Vec<String>>().join("\n")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <broker> [command] [arguments...]", args[0]);
        println!("Available brokers: binance, coinbase, kraken, kucoin");
        return Ok(());
    }

    let broker = &args[1];
    let configs = brokers::load_configs()?;
    let config = configs.get(broker).expect("Broker configuration not found");

    if args.len() == 2 {
        println!("Available commands for {}:", broker);
        if let Some(commands) = config["commands"].as_array() {
            println!("{}", generate_help_for_broker(commands));
        } else {
            println!("No commands available for this broker.");
        }
        return Ok(());
    }

    let command_name = args[2].trim_start_matches('-');
    let command_args: Vec<String> = args.iter()
        .skip(3)
        .filter_map(|arg| {
            if !arg.starts_with("--") {
                Some(arg.clone())
            } else {
                None
            }
        })
        .collect();

    if let Some(command) = config["commands"].as_array().unwrap().iter().find(|&cmd| cmd["name"].as_str().map(|name| name.trim_start_matches('-')) == Some(command_name)) {
        if command_args.len() != command["arguments"].as_array().unwrap().len() {
            println!("Incorrect number of arguments. {}", generate_usage(command));
            return Ok(());
        }
        if let Err(e) = brokers::kraken::execute_command(command, &command_args) {
            eprintln!("Error executing command: {}", e);
            return Err(e.into());
        }
    } else {
        println!("Command '{}' not found. Available commands:\n{}", command_name, generate_help_for_broker(config["commands"].as_array().unwrap()));
    }

    Ok(())
}
