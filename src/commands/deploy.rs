use anyhow::Result;
use std::process::Command;
use crate::commands::network::{get_network_url, get_network_name};

pub async fn handle_deploy(network: &str, program_id: Option<&str>) -> Result<()> {
    let network_name = get_network_name(network);
    let network_url = get_network_url(network);
    
    println!("🚀 Deploying to {}...", network_name);
    println!("🌐 RPC URL: {}", network_url);

    let mut cmd = Command::new("solana");
    cmd.args(["program", "deploy"]);
    
    // Find the .so file in target/deploy
    let deploy_dir = std::path::Path::new("target/deploy");
    if !deploy_dir.exists() {
        println!("❌ No build artifacts found. Run 'starframe build' first.");
        std::process::exit(1);
    }

    let so_files: Vec<_> = std::fs::read_dir(deploy_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .map(|ext| ext == "so")
                .unwrap_or(false)
        })
        .collect();

    if so_files.is_empty() {
        println!("❌ No .so file found in target/deploy. Run 'starframe build' first.");
        std::process::exit(1);
    }

    if so_files.len() > 1 {
        println!("⚠️  Multiple .so files found. Using the first one: {}", so_files[0].path().display());
    }

    cmd.arg(so_files[0].path());

    if let Some(id) = program_id {
        cmd.args(["--program-id", id]);
        println!("🔄 Upgrading program: {}", id);
    } else {
        println!("📦 Deploying new program...");
    }

    cmd.args(["--url", network_url]);

    // Show deployment cost estimate for mainnet
    if network_name == "mainnet-beta" {
        println!("💰 Note: Mainnet deployment requires SOL for rent and fees");
        println!("💡 Tip: Test on devnet first with: starframe deploy --network devnet");
    }

    let output = cmd.output()?;

    if output.status.success() {
        println!("✅ Program deployed successfully to {}!", network_name);
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
        
        // Extract program ID from output if it's a new deployment
        if program_id.is_none() {
            if let Some(line) = output_str.lines().find(|line| line.contains("Program Id:")) {
                println!("🆔 Save your Program ID for future upgrades!");
                println!("   {}", line);
            }
        }
    } else {
        println!("❌ Deployment failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        
        if network_name == "localnet" {
            println!("💡 Tip: Make sure your local validator is running:");
            println!("   solana-test-validator");
        }
        
        std::process::exit(1);
    }

    Ok(())
}