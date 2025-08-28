use anyhow::Result;
use std::path::Path;
use crate::utils::{
    generate_program_id,
    get_current_program_name,
    extract_program_id_from_lib,
    extract_program_id_from_toml,
    update_program_id_in_lib,
    update_program_id_in_toml,
};

pub async fn handle_sync(from_lib: bool) -> Result<()> {
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

    // Get program name
    let program_name = get_current_program_name()?;

    println!("🔄 Syncing program IDs...");
    println!("📋 Program: {}", program_name);

    // Extract current program IDs
    let lib_program_id = extract_program_id_from_lib(&lib_rs_path)?;
    let toml_program_id = extract_program_id_from_toml(&starframe_toml_path, &program_name)?;

    println!("\n📊 Current Program IDs:");
    println!("   lib.rs:        {}", lib_program_id.as_deref().unwrap_or("Not found"));
    println!("   StarFrame.toml: {}", toml_program_id.as_deref().unwrap_or("Not found"));

    match (lib_program_id.as_ref(), toml_program_id.as_ref()) {
        (Some(lib_id), Some(toml_id)) if lib_id == toml_id => {
            println!("\n✅ Program IDs are already synchronized!");
            println!("🆔 Program ID: {}", lib_id);
            return Ok(());
        }
        (Some(lib_id), Some(toml_id)) => {
            println!("\n⚠️  Program IDs are out of sync!");
            
            let (source_id, _target_file, action) = if from_lib {
                (lib_id, "StarFrame.toml", "lib.rs → StarFrame.toml")
            } else {
                (toml_id, "src/lib.rs", "StarFrame.toml → src/lib.rs")
            };

            println!("🔄 Syncing: {}", action);
            println!("🆔 Using Program ID: {}", source_id);

            if from_lib {
                update_program_id_in_toml(&starframe_toml_path, &program_name, lib_id)?;
            } else {
                update_program_id_in_lib(&lib_rs_path, toml_id)?;
                println!("✅ Updated program ID in src/lib.rs");
            }
        }
        (Some(lib_id), None) => {
            println!("\n🔄 Program ID found in lib.rs but not in StarFrame.toml");
            println!("🔄 Syncing: lib.rs → StarFrame.toml");
            println!("🆔 Using Program ID: {}", lib_id);
            update_program_id_in_toml(&starframe_toml_path, &program_name, lib_id)?;
        }
        (None, Some(toml_id)) => {
            println!("\n🔄 Program ID found in StarFrame.toml but not in lib.rs");
            println!("🔄 Syncing: StarFrame.toml → lib.rs");
            println!("🆔 Using Program ID: {}", toml_id);
            update_program_id_in_lib(&lib_rs_path, toml_id)?;
            println!("✅ Updated program ID in src/lib.rs");
        }
        (None, None) => {
            println!("\n❌ No program IDs found in either file!");
            println!("🔑 Generating new program ID...");
            
            let new_program_id = generate_program_id();
            println!("🆔 New Program ID: {}", new_program_id);

            // Update both files
            match update_program_id_in_lib(&lib_rs_path, &new_program_id) {
                Ok(()) => println!("✅ Updated program ID in src/lib.rs"),
                Err(e) => println!("⚠️  Could not update src/lib.rs: {}", e),
            }

            match update_program_id_in_toml(&starframe_toml_path, &program_name, &new_program_id) {
                Ok(()) => {},
                Err(e) => println!("⚠️  Could not update StarFrame.toml: {}", e),
            }
        }
    }

    // Verify sync
    let final_lib_id = extract_program_id_from_lib(&lib_rs_path)?;
    let final_toml_id = extract_program_id_from_toml(&starframe_toml_path, &program_name)?;

    match (final_lib_id.as_ref(), final_toml_id.as_ref()) {
        (Some(lib_id), Some(toml_id)) if lib_id == toml_id => {
            println!("\n🎉 Program IDs successfully synchronized!");
            println!("🆔 Final Program ID: {}", lib_id);
        }
        _ => {
            println!("\n⚠️  Sync may not be complete. Please check both files manually.");
        }
    }

    println!("\n🎯 Next steps:");
    println!("   1. Review the updated program IDs");
    println!("   2. Rebuild your program: starframe build");
    println!("   3. Update any client code with the program ID");

    Ok(())
}