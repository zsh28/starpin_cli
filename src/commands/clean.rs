use anyhow::Result;
use std::path::Path;

pub async fn handle_clean() -> Result<()> {
    println!("🧹 Cleaning Star Frame project artifacts...");
    
    if !Path::new("Cargo.toml").exists() {
        println!("❌ No Cargo.toml found. Run this command in a Star Frame project directory.");
        std::process::exit(1);
    }

    let mut cleaned_items: Vec<String> = Vec::new();

    if Path::new("target").exists() {
        if let Ok(entries) = std::fs::read_dir("target") {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                let name = path.file_name().unwrap().to_string_lossy();
                
                if name == "deploy" {
                    if let Err(_) = std::fs::remove_dir_all(&path) {
                        println!("⚠️  Failed to remove target/deploy directory");
                    } else {
                        cleaned_items.push("target/deploy/".to_string());
                    }
                } else if name == "idl" {
                    if let Err(_) = std::fs::remove_dir_all(&path) {
                        println!("⚠️  Failed to remove target/idl directory");
                    } else {
                        cleaned_items.push("target/idl/".to_string());
                    }
                } else if name.ends_with(".so") || name.ends_with(".json") {
                    if let Err(_) = std::fs::remove_file(&path) {
                        println!("⚠️  Failed to remove {}", path.display());
                    } else {
                        cleaned_items.push(format!("target/{}", name));
                    }
                }
            }
        }

        if let Ok(entries) = std::fs::read_dir("target/debug") {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() && path.file_name().unwrap().to_string_lossy().contains("build") {
                    if let Err(_) = std::fs::remove_dir_all(&path) {
                        println!("⚠️  Failed to remove {}", path.display());
                    } else {
                        cleaned_items.push("debug build artifacts".to_string());
                    }
                }
            }
        }

        if let Ok(entries) = std::fs::read_dir("target/release") {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() && path.file_name().unwrap().to_string_lossy().contains("build") {
                    if let Err(_) = std::fs::remove_dir_all(&path) {
                        println!("⚠️  Failed to remove {}", path.display());
                    } else {
                        cleaned_items.push("release build artifacts".to_string());
                    }
                }
            }
        }
    }

    if Path::new("node_modules").exists() {
        println!("🗑️  Removing node_modules...");
        if let Err(_) = std::fs::remove_dir_all("node_modules") {
            println!("⚠️  Failed to remove node_modules directory");
        } else {
            cleaned_items.push("node_modules/".to_string());
        }
    }

    if Path::new("coverage").exists() {
        if let Err(_) = std::fs::remove_dir_all("coverage") {
            println!("⚠️  Failed to remove coverage directory");
        } else {
            cleaned_items.push("coverage/".to_string());
        }
    }

    if cleaned_items.is_empty() {
        println!("✨ Project is already clean!");
    } else {
        println!("✅ Cleaned the following artifacts:");
        for item in cleaned_items {
            println!("   - {}", item);
        }
        println!("🔑 Program keypairs preserved");
    }

    Ok(())
}