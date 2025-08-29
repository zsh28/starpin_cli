use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;
use solana_sdk::signer::{keypair::Keypair, Signer};
use serde::{Deserialize, Serialize};
use semver::Version;

pub fn project_name_validator(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.starts_with('_')
}

/// Generate a random program ID using actual Ed25519 keypair
pub fn generate_program_id() -> String {
    let keypair = Keypair::new();
    keypair.pubkey().to_string()
}

/// Extract program ID from lib.rs file
pub fn extract_program_id_from_lib(lib_path: &Path) -> Result<Option<String>> {
    if !lib_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(lib_path)?;
    
    // Look for program ID in different formats
    for line in content.lines() {
        if line.trim().starts_with("id = ") {
            if let Some(id_start) = line.find('"') {
                if let Some(id_end) = line.rfind('"') {
                    if id_start != id_end {
                        let program_id = &line[id_start + 1..id_end];
                        return Ok(Some(program_id.to_string()));
                    }
                }
            }
        }
    }
    
    Ok(None)
}

/// Update program ID in lib.rs file
pub fn update_program_id_in_lib(lib_path: &Path, new_program_id: &str) -> Result<()> {
    if !lib_path.exists() {
        return Err(anyhow!("lib.rs not found at {}", lib_path.display()));
    }

    let content = fs::read_to_string(lib_path)?;
    let mut updated_content = String::new();
    let mut found_and_updated = false;

    for line in content.lines() {
        if line.trim().starts_with("id = ") {
            // Replace the program ID in this line
            if let Some(id_start) = line.find('"') {
                if let Some(id_end) = line.rfind('"') {
                    if id_start != id_end {
                        let before = &line[..id_start + 1];
                        let after = &line[id_end..];
                        updated_content.push_str(&format!("{}{}{}\n", before, new_program_id, after));
                        found_and_updated = true;
                        continue;
                    }
                }
            }
        }
        updated_content.push_str(line);
        updated_content.push('\n');
    }

    if !found_and_updated {
        return Err(anyhow!("Could not find program ID declaration in lib.rs"));
    }

    fs::write(lib_path, updated_content)?;
    Ok(())
}

/// Extract program ID from Starpin.toml
pub fn extract_program_id_from_toml(toml_path: &Path, program_name: &str) -> Result<Option<String>> {
    if !toml_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(toml_path)?;
    
    // Look for program ID in Starpin.toml
    for line in content.lines() {
        if line.trim().starts_with(&format!("{} = ", program_name)) {
            if let Some(id_start) = line.find('"') {
                if let Some(id_end) = line.rfind('"') {
                    if id_start != id_end {
                        let program_id = &line[id_start + 1..id_end];
                        return Ok(Some(program_id.to_string()));
                    }
                }
            }
        }
    }
    
    Ok(None)
}

/// Update program ID in Starpin.toml
pub fn update_program_id_in_toml(toml_path: &Path, program_name: &str, new_program_id: &str) -> Result<()> {
    if !toml_path.exists() {
        return Err(anyhow!("Starpin.toml not found at {}", toml_path.display()));
    }

    let content = fs::read_to_string(toml_path)?;
    let mut updated_content = String::new();
    let mut updates_made = 0;

    for line in content.lines() {
        if line.trim().starts_with(&format!("{} = ", program_name)) {
            // Replace the program ID in this line
            if let Some(id_start) = line.find('"') {
                if let Some(id_end) = line.rfind('"') {
                    if id_start != id_end {
                        let before = &line[..id_start + 1];
                        let after = &line[id_end..];
                        updated_content.push_str(&format!("{}{}{}\n", before, new_program_id, after));
                        updates_made += 1;
                        continue;
                    }
                }
            }
        }
        updated_content.push_str(line);
        updated_content.push('\n');
    }

    if updates_made == 0 {
        return Err(anyhow!("Could not find program '{}' declaration in Starpin.toml", program_name));
    }

    fs::write(toml_path, updated_content)?;
    println!("✅ Updated {} program ID entries in Starpin.toml", updates_made);
    Ok(())
}

/// Get current directory name as program name
pub fn get_current_program_name() -> Result<String> {
    let current_dir = std::env::current_dir()?;
    let dir_name = current_dir
        .file_name()
        .ok_or_else(|| anyhow!("Could not get current directory name"))?
        .to_str()
        .ok_or_else(|| anyhow!("Current directory name is not valid UTF-8"))?;
    Ok(dir_name.replace('-', "_"))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrateVersion {
    pub num: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrateInfo {
    pub versions: Vec<CrateVersion>,
}

/// Fetch the latest version of a crate from crates.io
pub async fn fetch_latest_crate_version(crate_name: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    
    let response = client
        .get(&url)
        .header("User-Agent", "starframe-cli")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch crate info for '{}': HTTP {}", crate_name, response.status()));
    }

    let crate_info: CrateInfo = response.json().await?;
    
    if crate_info.versions.is_empty() {
        return Err(anyhow!("No versions found for crate '{}'", crate_name));
    }

    // Find the latest non-prerelease version
    let mut latest_stable = None;
    let mut latest_version = None;

    for version in &crate_info.versions {
        if let Ok(ver) = Version::parse(&version.num) {
            if latest_version.is_none() || ver > *latest_version.as_ref().unwrap() {
                latest_version = Some(ver.clone());
                
                // Check if this is a stable version (no pre-release)
                if ver.pre.is_empty() && latest_stable.is_none() {
                    latest_stable = Some(ver.clone());
                }
            }
        }
    }

    // Prefer stable version, fallback to latest
    let chosen_version = latest_stable.or(latest_version)
        .ok_or_else(|| anyhow!("No valid versions found for crate '{}'", crate_name))?;

    Ok(chosen_version.to_string())
}

/// Get dependency versions for templates
pub async fn get_dependency_versions(star_frame_version: Option<&str>) -> Result<DependencyVersions> {
    let star_frame_version = if let Some(version) = star_frame_version {
        version.to_string()
    } else {
        println!("🔍 Fetching latest Star Frame version...");
        match fetch_latest_crate_version("star_frame").await {
            Ok(version) => {
                println!("✅ Found Star Frame version: {}", version);
                version
            }
            Err(_) => {
                println!("⚠️  Could not fetch latest version, using fallback");
                "0.1.0".to_string()
            }
        }
    };

    // You can extend this to fetch other common dependency versions
    Ok(DependencyVersions {
        star_frame: star_frame_version,
        solana_program: "1.18".to_string(), // Keep stable for compatibility
        spl_token: "4.0".to_string(),
        spl_associated_token_account: "2.3".to_string(),
    })
}

#[derive(Debug, Clone)]
pub struct DependencyVersions {
    pub star_frame: String,
    pub solana_program: String,
    #[allow(dead_code)]
    pub spl_token: String,
    #[allow(dead_code)]
    pub spl_associated_token_account: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_project_names() {
        assert!(project_name_validator("my_project"));
        assert!(project_name_validator("my-project"));
        assert!(project_name_validator("myproject"));
        assert!(project_name_validator("my_project_123"));
    }

    #[test]
    fn test_invalid_project_names() {
        assert!(!project_name_validator(""));
        assert!(!project_name_validator("-project"));
        assert!(!project_name_validator("project-"));
        assert!(!project_name_validator("_project"));
        assert!(!project_name_validator("my project"));
        assert!(!project_name_validator("my/project"));
    }
}