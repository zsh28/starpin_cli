use anyhow::Result;
use std::process::Command;
use crate::commands::network::{get_network_url, get_network_name};

pub async fn handle_test(filter: Option<&str>, network: &str) -> Result<()> {
    let network_name = get_network_name(network);
    let network_url = get_network_url(network);
    
    println!("🧪 Running Star Frame tests...");
    println!("🌐 Network: {} ({})", network_name, network_url);
    
    let mut cmd = Command::new("cargo");
    cmd.args(["test"]);
    cmd.env("SOLANA_NETWORK", network_name);
    cmd.env("SOLANA_RPC_URL", network_url);
    
    // Enable test helpers feature if available
    if has_test_helpers_feature() {
        cmd.args(["--features", "test_helpers"]);
        println!("🔧 Enabling test_helpers feature...");
    }
    
    if let Some(filter) = filter {
        cmd.arg(filter);
        println!("🔍 Running tests with filter: {}", filter);
    }

    if network_name == "localnet" {
        println!("💡 Testing against localnet - make sure your validator is running:");
        println!("   solana-test-validator");
    }

    let output = cmd.output()?;

    if output.status.success() {
        println!("✅ All tests passed!");
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim().is_empty() {
            println!("{}", stdout);
        }
    } else {
        println!("❌ Tests failed:");
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}", stderr);
        
        // Provide helpful hints for common test failures
        if stderr.contains("Connection refused") {
            println!("\n💡 Tip: If testing against localnet, start your validator:");
            println!("   solana-test-validator");
        }
        
        std::process::exit(1);
    }

    Ok(())
}

fn has_test_helpers_feature() -> bool {
    if let Ok(cargo_content) = std::fs::read_to_string("Cargo.toml") {
        cargo_content.contains("test_helpers")
    } else {
        false
    }
}