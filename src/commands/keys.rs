use anyhow::Result;
use std::path::Path;
use crate::utils::{
    generate_program_id, 
    get_current_program_name, 
    update_program_id_in_lib, 
    update_program_id_in_toml
};

pub async fn handle_keys(program_name: Option<&str>) -> Result<()> {
    // Check if we're in a Star Frame project
    let starframe_toml_path = Path::new("StarFrame.toml");
    let lib_rs_path = Path::new("src/lib.rs");

    if !starframe_toml_path.exists() {
        println!("❌ StarFrame.toml not found. Make sure you're in a Star Frame project directory.");
        std::process::exit(1);
    }

    if !lib_rs_path.exists() {
        println!("❌ src/lib.rs not found. Make sure you're in a Star Frame project directory.");
        std::process::exit(1);
    }

    // Determine program name
    let program_name = match program_name {
        Some(name) => name.replace('-', "_"),
        None => get_current_program_name()?,
    };

    // Generate new program ID
    let new_program_id = generate_program_id();

    println!("🔑 Generating new program keypair...");
    println!("📋 Program: {}", program_name);
    println!("🆔 New Program ID: {}", new_program_id);

    // Update lib.rs
    match update_program_id_in_lib(&lib_rs_path, &new_program_id) {
        Ok(()) => println!("✅ Updated program ID in src/lib.rs"),
        Err(e) => {
            println!("⚠️  Could not update src/lib.rs: {}", e);
            println!("   Please manually update the program ID in your lib.rs file");
        }
    }

    // Update StarFrame.toml
    match update_program_id_in_toml(&starframe_toml_path, &program_name, &new_program_id) {
        Ok(()) => {},
        Err(e) => {
            println!("⚠️  Could not update StarFrame.toml: {}", e);
            println!("   Please manually update the program ID in your StarFrame.toml file");
        }
    }

    println!("\n🎯 Next steps:");
    println!("   1. Review the updated program IDs in both files");
    println!("   2. Rebuild your program: starframe build");
    println!("   3. Update any client code with the new program ID");
    println!("\n💡 Tip: Use 'starframe sync' to verify both files are synchronized");

    Ok(())
}