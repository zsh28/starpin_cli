use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;
use crate::utils::get_dependency_versions;

pub async fn handle_update(star_frame_version: Option<&str>, dry_run: bool) -> Result<()> {
    let cargo_toml_path = Path::new("Cargo.toml");
    
    if !cargo_toml_path.exists() {
        return Err(anyhow!("Cargo.toml not found. Are you in a Star Frame project directory?"));
    }

    println!("🔍 Checking for dependency updates...");

    // Get latest versions
    let versions = get_dependency_versions(star_frame_version).await?;

    // Read current Cargo.toml
    let cargo_content = fs::read_to_string(cargo_toml_path)?;
    
    // Parse and analyze current dependencies
    let mut updates_needed = Vec::new();
    let mut updated_content = cargo_content.clone();

    // Check for Star Frame dependency
    if cargo_content.contains("star_frame") {
        // Extract current version (simplified parsing)
        if let Some(current_version) = extract_dependency_version(&cargo_content, "star_frame") {
            if current_version != versions.star_frame {
                updates_needed.push(format!("star_frame: {} → {}", current_version, versions.star_frame));
                
                if !dry_run {
                    updated_content = update_dependency_version(&updated_content, "star_frame", &versions.star_frame);
                }
            } else {
                println!("✅ star_frame is already up to date ({})", current_version);
            }
        }
    }

    // Check for Solana dependencies
    if cargo_content.contains("solana-program") {
        if let Some(current_version) = extract_dependency_version(&cargo_content, "solana-program") {
            if current_version != versions.solana_program {
                updates_needed.push(format!("solana-program: {} → {}", current_version, versions.solana_program));
                
                if !dry_run {
                    updated_content = update_dependency_version(&updated_content, "solana-program", &versions.solana_program);
                }
            }
        }
    }

    if updates_needed.is_empty() {
        println!("✅ All dependencies are up to date!");
        return Ok(());
    }

    if dry_run {
        println!("🔍 Updates available (dry run):");
        for update in &updates_needed {
            println!("  📦 {}", update);
        }
        println!("\nRun without --dry-run to apply these updates.");
    } else {
        println!("🔄 Applying updates:");
        for update in &updates_needed {
            println!("  📦 {}", update);
        }

        // Write updated Cargo.toml
        fs::write(cargo_toml_path, updated_content)?;
        
        println!("\n✅ Dependencies updated successfully!");
        println!("💡 Run 'cargo update' to refresh your lock file.");
    }

    Ok(())
}

fn extract_dependency_version(content: &str, dep_name: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(dep_name) && line.contains("version") {
            // Try to extract version from patterns like:
            // dep_name = "version"
            // dep_name = { version = "version" }
            if let Some(start) = line.find("version") {
                let version_part = &line[start..];
                if let Some(quote_start) = version_part.find('"') {
                    if let Some(quote_end) = version_part[quote_start + 1..].find('"') {
                        return Some(version_part[quote_start + 1..quote_start + 1 + quote_end].to_string());
                    }
                }
            }
        }
    }
    None
}

fn update_dependency_version(content: &str, dep_name: &str, new_version: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut updated_lines = Vec::new();

    for line in lines {
        if line.trim().starts_with(dep_name) && line.contains("version") {
            // Replace the version in this line
            if let Some(start) = line.find("version") {
                let before_version = &line[..start + 7]; // "version"
                let after_version_start = line[start + 7..].find('"').unwrap_or(0) + start + 7;
                let after_version_end = line[after_version_start + 1..].find('"').unwrap_or(0) + after_version_start + 1;
                let after_version = &line[after_version_end..];
                
                let updated_line = format!("{} = \"{}{}", before_version, new_version, after_version);
                updated_lines.push(updated_line);
            } else {
                updated_lines.push(line.to_string());
            }
        } else {
            updated_lines.push(line.to_string());
        }
    }

    updated_lines.join("\n")
}