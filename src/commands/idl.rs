use anyhow::Result;
use std::process::Command;
use std::path::Path;

pub async fn handle_idl(output: &str) -> Result<()> {
    println!("📋 Generating IDL...");
    
    let output_dir = Path::new(output);
    std::fs::create_dir_all(output_dir)?;

    let mut cmd = Command::new("cargo");
    cmd.args(["build"]);
    cmd.env("STAR_FRAME_IDL_OUTPUT", output);
    cmd.env("STAR_FRAME_GENERATE_IDL", "true");

    let build_output = cmd.output()?;

    if !build_output.status.success() {
        println!("❌ IDL generation failed during build:");
        println!("{}", String::from_utf8_lossy(&build_output.stderr));
        std::process::exit(1);
    }

    println!("✅ IDL generated successfully!");
    println!("📁 Location: {}", output_dir.display());

    // List generated IDL files
    if output_dir.exists() {
        let idl_files: Vec<_> = std::fs::read_dir(output_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension()
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
            })
            .collect();

        if !idl_files.is_empty() {
            println!("📄 Generated IDL files:");
            for file in idl_files {
                println!("   - {}", file.file_name().to_string_lossy());
            }
        }
    }

    Ok(())
}