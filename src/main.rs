use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod templates;
mod utils;

#[derive(Parser)]
#[command(name = "starpin")]
#[command(about = "A CLI tool for creating and managing Star Frame Solana programs")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize a new Star Frame project")]
    Init {
        #[arg(help = "Name of the project")]
        name: String,
        #[arg(long, help = "Template to use (counter, simple_counter, marketplace)", default_value = "counter")]
        template: String,
        #[arg(long, help = "Directory to create project in", default_value = ".")]
        path: String,
        #[arg(long, help = "Star Frame version to use (e.g., 0.1.0, latest)")]
        version: Option<String>,
    },
    #[command(about = "Build the Star Frame program")]
    Build {
        #[arg(long, help = "Network to build for (localnet, devnet, mainnet)", default_value = "localnet")]
        network: String,
        #[arg(long, help = "Skip IDL generation", default_value = "false")]
        skip_idl: bool,
    },
    #[command(about = "Test the Star Frame program")]
    Test {
        #[arg(long, help = "Run tests with specific filter")]
        filter: Option<String>,
        #[arg(long, help = "Network to test against (localnet, devnet, mainnet)", default_value = "localnet")]
        network: String,
    },
    #[command(about = "Deploy the Star Frame program")]
    Deploy {
        #[arg(long, help = "Network to deploy to (localnet, devnet, mainnet)", default_value = "devnet")]
        network: String,
        #[arg(long, help = "Program ID to upgrade")]
        program_id: Option<String>,
    },
    #[command(about = "Generate IDL for the program")]
    Idl {
        #[arg(long, help = "Output directory for IDL", default_value = "target/idl")]
        output: String,
    },
    #[command(about = "Show current network configuration")]
    Network,
    #[command(about = "Generate a new program keypair")]
    Keys {
        #[arg(long, help = "Program name to update (defaults to current directory name)")]
        program: Option<String>,
    },
    #[command(about = "Sync program IDs between Starpin.toml and lib.rs")]
    Sync {
        #[arg(long, help = "Use program ID from lib.rs instead of generating new one", default_value = "false")]
        from_lib: bool,
    },
    #[command(about = "Update dependencies to latest versions")]
    Update {
        #[arg(long, help = "Update to specific Star Frame version")]
        star_frame: Option<String>,
        #[arg(long, help = "Dry run - show what would be updated without making changes", default_value = "false")]
        dry_run: bool,
    },
    #[command(about = "Remove all artifacts from the generated directories except program keypairs")]
    Clean,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, template, path, version } => {
            commands::init::handle_init(&name, &template, &path, version.as_deref()).await
        }
        Commands::Build { network, skip_idl } => {
            commands::build::handle_build(&network, skip_idl).await
        }
        Commands::Test { filter, network } => {
            commands::test::handle_test(filter.as_deref(), &network).await
        }
        Commands::Deploy { network, program_id } => {
            commands::deploy::handle_deploy(&network, program_id.as_deref()).await
        }
        Commands::Idl { output } => {
            commands::idl::handle_idl(&output).await
        }
        Commands::Network => {
            commands::network::handle_network().await
        }
        Commands::Keys { program } => {
            commands::keys::handle_keys(program.as_deref()).await
        }
        Commands::Sync { from_lib } => {
            commands::sync::handle_sync(from_lib).await
        }
        Commands::Update { star_frame, dry_run } => {
            commands::update::handle_update(star_frame.as_deref(), dry_run).await
        }
        Commands::Clean => {
            commands::clean::handle_clean().await
        }
    }
}
