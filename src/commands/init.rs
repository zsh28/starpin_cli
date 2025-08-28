use anyhow::{Result, anyhow};
use std::path::Path;
use crate::templates::Template;
use crate::utils::{project_name_validator, get_dependency_versions};

pub async fn handle_init(name: &str, template: &str, path: &str, star_frame_version: Option<&str>) -> Result<()> {
    if !project_name_validator(name) {
        return Err(anyhow!("Invalid project name. Use only alphanumeric characters, hyphens, and underscores."));
    }

    let project_path = Path::new(path).join(name);
    
    if project_path.exists() {
        return Err(anyhow!("Directory '{}' already exists", project_path.display()));
    }

    println!("🚀 Creating Star Frame project: {}", name);
    println!("📁 Location: {}", project_path.display());
    println!("📋 Template: {}", template);

    // Get dependency versions
    let versions = get_dependency_versions(star_frame_version).await?;
    println!("📦 Using Star Frame: {}", versions.star_frame);

    let template_impl: Box<dyn Template> = match template {
        "counter" => Box::new(crate::templates::counter::CounterTemplate::new()),
        "simple_counter" | "simple-counter" => Box::new(crate::templates::simple_counter::SimpleCounterTemplate::new()),
        "marketplace" => Box::new(crate::templates::marketplace::MarketplaceTemplate::new()),
        _ => return Err(anyhow!("Unknown template: {}. Available templates: counter, simple_counter, marketplace", template)),
    };

    template_impl.generate_with_versions(&project_path, name, &versions)?;

    println!("✅ Project '{}' created successfully!", name);
    println!("\n📝 Next steps:");
    println!("   cd {}", name);
    println!("   starframe build                    # Build for localnet");
    println!("   starframe test                     # Run tests");
    println!("   starframe deploy --network devnet  # Deploy to devnet");
    println!("\n🌐 Available networks: localnet, devnet, mainnet");
    println!("📋 Configuration file: StarFrame.toml");
    
    Ok(())
}