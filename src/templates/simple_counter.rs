use super::Template;
use anyhow::Result;
use std::path::Path;
use std::fs;
use crate::utils::{generate_program_id, DependencyVersions};

pub struct SimpleCounterTemplate;

impl SimpleCounterTemplate {
    pub fn new() -> Self {
        Self
    }
}

impl Template for SimpleCounterTemplate {
    fn generate(&self, project_path: &Path, project_name: &str) -> Result<()> {
        // Use default versions for backward compatibility
        let versions = DependencyVersions {
            star_frame: "0.1.0".to_string(),
            solana_program: "1.18".to_string(),
            spl_token: "4.0".to_string(),
            spl_associated_token_account: "2.3".to_string(),
        };
        self.generate_with_versions(project_path, project_name, &versions)
    }

    fn generate_with_versions(&self, project_path: &Path, project_name: &str, versions: &DependencyVersions) -> Result<()> {
        fs::create_dir_all(project_path)?;
        fs::create_dir_all(project_path.join("src"))?;
        fs::create_dir_all(project_path.join("tests"))?;

        // Generate actual Solana keypair program ID
        let program_id = generate_program_id();

        // Create Cargo.toml
        let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "A simple Star Frame counter program"

[dependencies]
star_frame = {{ version = "{}", features = ["idl"] }}
bytemuck = {{ version = "1.18", features = ["derive"] }}
borsh = {{ version = "1.5", features = ["derive"] }}
anyhow = "1.0"

[lib]
crate-type = ["cdylib", "lib"]

[features]
default = []
test_helpers = ["star_frame/test_helpers"]
idl = ["star_frame/idl"]

[package.metadata.solana]
program-id = "{}"

[dev-dependencies]
tokio = {{ version = "1.0", features = ["macros", "rt-multi-thread"] }}
"#, project_name, versions.star_frame, program_id);

        fs::write(project_path.join("Cargo.toml"), cargo_toml)?;

        // Create lib.rs (based on simple_counter example)
        let lib_rs = format!(r#"use star_frame::{{anyhow::ensure, prelude::*}};

#[derive(StarFrameProgram)]
#[program(
    instruction_set = CounterInstructionSet,
    id = "{}"
)]
pub struct CounterProgram;

#[derive(InstructionSet)]
pub enum CounterInstructionSet {{
    Initialize(Initialize),
    Increment(Increment),
}}

#[derive(Align1, Pod, Zeroable, Default, Copy, Clone, Debug, Eq, PartialEq, ProgramAccount)]
#[program_account(seeds = CounterSeeds)]
#[repr(C, packed)]
pub struct CounterAccount {{
    pub authority: Pubkey,
    pub count: u64,
}}

#[derive(Debug, GetSeeds, Clone)]
#[get_seeds(seed_const = b"COUNTER")]
pub struct CounterSeeds {{
    pub authority: Pubkey,
}}

impl AccountValidate<&Pubkey> for CounterAccount {{
    fn validate_account(self_ref: &Self::Ref<'_>, arg: &Pubkey) -> Result<()> {{
        ensure!(arg == &self_ref.authority, "Incorrect authority");
        Ok(())
    }}
}}

#[derive(BorshSerialize, BorshDeserialize, Debug, InstructionArgs)]
pub struct Initialize {{
    #[ix_args(&run)]
    pub start_at: Option<u64>,
}}

#[derive(AccountSet)]
pub struct InitializeAccounts {{
    #[validate(funder)]
    pub authority: Signer<Mut<SystemAccount>>,
    #[validate(arg = (
        Create(()),
        Seeds(CounterSeeds {{ authority: *self.authority.pubkey() }}),
    ))]
    #[idl(arg = Seeds(FindCounterSeeds {{ authority: seed_path("authority") }}))]
    pub counter: Init<Seeded<Account<CounterAccount>>>,
    pub system_program: Program<System>,
}}

impl StarFrameInstruction for Initialize {{
    type ReturnType = ();
    type Accounts<'b, 'c> = InitializeAccounts;

    fn process(
        accounts: &mut Self::Accounts<'_, '_>,
        start_at: &Option<u64>,
        _ctx: &mut Context,
    ) -> Result<Self::ReturnType> {{
        **accounts.counter.data_mut()? = CounterAccount {{
            authority: *accounts.authority.pubkey(),
            count: start_at.unwrap_or(0),
        }};
        Ok(())
    }}
}}

#[derive(BorshSerialize, BorshDeserialize, Debug, Copy, Clone, InstructionArgs)]
pub struct Increment;

#[derive(AccountSet, Debug)]
pub struct IncrementAccounts {{
    pub authority: Signer,
    #[validate(arg = self.authority.pubkey())]
    pub counter: Mut<ValidatedAccount<CounterAccount>>,
}}

impl StarFrameInstruction for Increment {{
    type ReturnType = ();
    type Accounts<'b, 'c> = IncrementAccounts;

    fn process(
        accounts: &mut Self::Accounts<'_, '_>,
        _run_arg: Self::RunArg<'_>,
        _ctx: &mut Context,
    ) -> Result<Self::ReturnType> {{
        let mut counter = accounts.counter.data_mut()?;
        counter.count += 1;
        Ok(())
    }}
}}

// IDL seed structs for code generation
#[cfg(feature = "idl")]
#[derive(Debug, GetSeeds, Clone)]
#[get_seeds(seed_const = b"COUNTER")]
pub struct FindCounterSeeds {{
    pub authority: SeedPath,
}}
"#, program_id);

        fs::write(project_path.join("src").join("lib.rs"), lib_rs)?;

        // Create test file
        let test_rs = format!(r#"use {}::*;

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_counter_initialization() {{
        println!("Simple counter initialization test");
    }}

    #[test]
    fn test_counter_increment() {{
        println!("Simple counter increment test");
    }}

    #[test]
    fn test_authority_validation() {{
        println!("Simple counter authority validation test");
    }}
}}
"#, project_name.replace('-', "_"));

        fs::write(project_path.join("tests").join("simple_counter.rs"), test_rs)?;

        // Create README.md
        let readme = format!(r#"# {} - Simple Star Frame Counter Program

A minimal counter program built with the Star Frame framework for Solana.

## Features

- ✅ Initialize counter with optional starting value
- ✅ Increment counter value
- ✅ Authority-based access control
- ✅ Type-safe account validation
- ✅ Minimal, easy-to-understand code

## Getting Started

### Prerequisites

- Rust 1.84.1+
- Solana CLI tools
- Star Frame CLI

### Building

```bash
starpin build
```

### Testing

```bash
starpin test
```

### Deploying

To devnet:
```bash
starpin deploy --network devnet
```

To mainnet:
```bash
starpin deploy --network mainnet
```

### Generate IDL

```bash
starpin idl
```

## Program Structure

- `CounterAccount`: Simple program account storing authority and count
- `Initialize`: Initialize a new counter with optional starting value
- `Increment`: Increment the counter value by 1

## Security Features

- Authority validation for increment operations
- Type-safe account handling
- Compile-time validation

## Program ID

```
{}
```

## Example Usage

This is a basic counter program that demonstrates:
- Program Derived Addresses (PDAs) with seeds
- Account validation and authorization
- Simple state management
- Star Frame instruction patterns

Perfect for learning Star Frame development!
"#, project_name, program_id);

        fs::write(project_path.join("README.md"), readme)?;

        // Create .gitignore
        let gitignore = r#"# Rust
target/
Cargo.lock

# Solana
keypairs/
.anchor/

# IDEs
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db

# Environment
.env
"#;

        fs::write(project_path.join(".gitignore"), gitignore)?;

        // Create Starpin.toml
        let starpin_toml = format!(r#"[toolchain]

[features]
resolution = true
skip-lint = false

[programs.localnet]
{} = "{}"

[programs.devnet]
{} = "{}"

[programs.mainnet]
{} = "{}"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"

[scripts]
build = "starpin build"
test = "starpin test"
deploy = "starpin deploy"
"#, 
            project_name.replace('-', "_"), program_id,
            project_name.replace('-', "_"), program_id,
            project_name.replace('-', "_"), program_id
        );

        fs::write(project_path.join("Starpin.toml"), starpin_toml)?;

        Ok(())
    }
}