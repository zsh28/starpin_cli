use anyhow::Result;
use std::process::Command;

pub async fn handle_network() -> Result<()> {
    println!("🌐 Network Configuration");
    
    // Check current Solana config
    let output = Command::new("solana")
        .args(["config", "get"])
        .output();

    match output {
        Ok(result) if result.status.success() => {
            let config_output = String::from_utf8_lossy(&result.stdout);
            println!("{}", config_output);
        }
        Ok(result) => {
            println!("⚠️  Could not get Solana config:");
            println!("{}", String::from_utf8_lossy(&result.stderr));
            show_default_networks();
        }
        Err(_) => {
            println!("⚠️  Solana CLI not found. Install with:");
            println!("   sh -c \"$(curl -sSfL https://release.solana.com/stable/install)\"");
            show_default_networks();
        }
    }

    Ok(())
}

fn show_default_networks() {
    println!("\n📋 Available Networks:");
    println!("   🏠 localnet    - Local validator (http://127.0.0.1:8899)");
    println!("   🚧 devnet      - Solana devnet (https://api.devnet.solana.com)");
    println!("   🌍 mainnet     - Solana mainnet (https://api.mainnet-beta.solana.com)");
    println!("\n💡 Set network with: solana config set --url <network>");
}

pub fn get_network_url(network: &str) -> &str {
    match network {
        "localnet" | "localhost" => "http://127.0.0.1:8899",
        "devnet" => "https://api.devnet.solana.com",
        "mainnet" | "mainnet-beta" => "https://api.mainnet-beta.solana.com",
        _ => {
            eprintln!("⚠️  Unknown network: {}. Using devnet.", network);
            "https://api.devnet.solana.com"
        }
    }
}

pub fn get_network_name(network: &str) -> &str {
    match network {
        "localnet" | "localhost" => "localnet",
        "devnet" => "devnet", 
        "mainnet" | "mainnet-beta" => "mainnet-beta",
        _ => {
            eprintln!("⚠️  Unknown network: {}. Using devnet.", network);
            "devnet"
        }
    }
}