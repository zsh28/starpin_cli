use anyhow::Result;
use std::process::Command;
use std::path::Path;
use crate::commands::network::{get_network_url, get_network_name};

pub async fn handle_build(network: &str, skip_idl: bool) -> Result<()> {
    let network_name = get_network_name(network);
    let network_url = get_network_url(network);
    
    println!("🔨 Building Star Frame program...");
    println!("🌐 Network: {} ({})", network_name, network_url);
    
    // Check if this is a Star Frame project
    if !Path::new("Cargo.toml").exists() {
        println!("❌ No Cargo.toml found. Run this command in a Star Frame project directory.");
        std::process::exit(1);
    }

    let mut cmd = Command::new("cargo");
    cmd.args(["build-sbf"]);
    cmd.env("SOLANA_NETWORK", network_name);
    cmd.env("SOLANA_RPC_URL", network_url);

    // Check if IDL feature is enabled and auto-generate IDL unless skipped
    if !skip_idl && should_generate_idl() {
        println!("📋 IDL generation enabled, will generate IDL after build...");
        cmd.env("STAR_FRAME_GENERATE_IDL", "true");
        cmd.env("STAR_FRAME_IDL_OUTPUT", "target/idl");
    }

    let output = cmd.output()?;

    if output.status.success() {
        println!("✅ Build completed successfully!");
        println!("📦 Program binary: target/deploy/");
        
        // Auto-generate IDL if enabled and not skipped
        if !skip_idl && should_generate_idl() {
            println!("📋 Generating IDL...");
            generate_idl().await?;
        }
    } else {
        println!("❌ Build failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    Ok(())
}

fn should_generate_idl() -> bool {
    // Check if Cargo.toml has star_frame dependency with idl feature
    if let Ok(cargo_content) = std::fs::read_to_string("Cargo.toml") {
        cargo_content.contains("star_frame") && 
        (cargo_content.contains(r#"features = ["idl"]"#) || 
         cargo_content.contains(r#"features = ['idl']"#) ||
         cargo_content.contains(r#""idl""#) ||
         cargo_content.contains(r#"'idl'"#))
    } else {
        false
    }
}

async fn generate_idl() -> Result<()> {
    // Create IDL directory if it doesn't exist
    std::fs::create_dir_all("target/idl")?;
    
    // Run cargo build with IDL generation environment variables
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--features", "idl"]);
    cmd.env("STAR_FRAME_GENERATE_IDL", "true");
    cmd.env("STAR_FRAME_IDL_OUTPUT", "target/idl");

    let output = cmd.output()?;

    if output.status.success() {
        // List generated IDL files
        if let Ok(entries) = std::fs::read_dir("target/idl") {
            let idl_files: Vec<_> = entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry.path().extension()
                        .map(|ext| ext == "json")
                        .unwrap_or(false)
                })
                .collect();

            if !idl_files.is_empty() {
                println!("✅ IDL generated successfully!");
                println!("📄 Generated files:");
                for file in idl_files {
                    println!("   - target/idl/{}", file.file_name().to_string_lossy());
                }
            } else {
                println!("⚠️  No IDL files generated. Check if your program exports IDL properly.");
            }
        }
    } else {
        println!("⚠️  IDL generation failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}