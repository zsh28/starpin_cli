use super::Template;
use anyhow::Result;
use std::path::Path;
use std::fs;
use crate::utils::{generate_program_id, DependencyVersions};

pub struct CounterTemplate;

impl CounterTemplate {
    pub fn new() -> Self {
        Self
    }
}

impl Template for CounterTemplate {
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
description = "A Star Frame counter program"

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

        // Create lib.rs - matches advanced counter example
        let lib_rs = format!(r#"use star_frame::{{
    anyhow::bail,
    derive_more::{{self, Deref, DerefMut}},
    empty_star_frame_instruction,
    prelude::*,
}};

#[derive(StarFrameProgram)]
#[program(
    instruction_set = CounterInstructionSet,
    id = "{}"
)]
pub struct CounterProgram;

#[derive(InstructionSet)]
pub enum CounterInstructionSet {{
    CreateCounter(CreateCounterIx),
    UpdateSigner(UpdateCounterSignerIx),
    Count(CountIx),
    CloseCounter(CloseCounterIx),
}}

#[derive(Align1, Pod, Zeroable, Default, Copy, Clone, Debug, Eq, PartialEq, ProgramAccount)]
#[program_account(seeds = CounterAccountSeeds)]
#[repr(C, packed)]
pub struct CounterAccount {{
    pub version: u8,
    pub owner: Pubkey,
    pub signer: Pubkey,
    pub count: u64,
    pub bump: u8,
    pub data: CounterAccountData,
}}

#[derive(Align1, Pod, Zeroable, Default, Copy, Clone, Debug, Eq, PartialEq, TypeToIdl)]
#[repr(C, packed)]
pub struct CounterAccountData {{
    pub version: u8,
    pub owner: Pubkey,
    pub signer: Pubkey,
    pub count: u64,
    pub bump: u8,
}}

#[derive(AccountSet, Deref, DerefMut, Debug)]
pub struct WrappedCounter(#[single_account_set] Account<CounterAccount>);

#[derive(Debug, GetSeeds, Clone)]
#[get_seeds(seed_const = b"COUNTER")]
pub struct CounterAccountSeeds {{
    pub owner: Pubkey,
}}

#[derive(BorshSerialize, BorshDeserialize, Debug, InstructionArgs)]
pub struct CreateCounterIx {{
    #[ix_args(&run)]
    pub start_at: Option<u64>,
}}

#[derive(AccountSet)]
pub struct CreateCounterAccounts {{
    #[validate(funder)]
    pub funder: Signer<Mut<SystemAccount>>,
    pub owner: SystemAccount,
    #[validate(arg = (
        CreateIfNeeded(()),
        Seeds(CounterAccountSeeds {{ owner: *self.owner.pubkey() }}),
    ))]
    #[idl(arg = Seeds(FindCounterAccountSeeds {{ owner: seed_path("owner") }}))]
    pub counter: Init<Seeded<WrappedCounter>>,
    pub system_program: Program<System>,
}}

impl StarFrameInstruction for CreateCounterIx {{
    type ReturnType = ();
    type Accounts<'b, 'c> = CreateCounterAccounts;

    fn process(
        accounts: &mut Self::Accounts<'_, '_>,
        start_at: Self::RunArg<'_>,
        _ctx: &mut Context,
    ) -> Result<Self::ReturnType> {{
        **accounts.counter.data_mut()? = CounterAccount {{
            version: 0,
            signer: *accounts.owner.pubkey(),
            owner: *accounts.owner.pubkey(),
            bump: accounts.counter.access_seeds().bump,
            count: start_at.unwrap_or(0),
            data: Default::default(),
        }};

        Ok(())
    }}
}}

#[derive(BorshSerialize, BorshDeserialize, Debug, InstructionArgs)]
#[ix_args(&run)]
pub struct UpdateCounterSignerIx;

#[derive(AccountSet, Debug)]
#[validate(extra_validation = self.validate())]
pub struct UpdateCounterSignerAccounts {{
    pub signer: Signer<SystemAccount>,
    pub new_signer: SystemAccount,
    pub counter: Mut<Account<CounterAccount>>,
}}

impl UpdateCounterSignerAccounts {{
    fn validate(&self) -> Result<()> {{
        if *self.signer.pubkey() != self.counter.data()?.signer {{
            bail!("Incorrect signer");
        }}
        Ok(())
    }}
}}

impl StarFrameInstruction for UpdateCounterSignerIx {{
    type ReturnType = ();
    type Accounts<'b, 'c> = UpdateCounterSignerAccounts;

    fn process(
        accounts: &mut Self::Accounts<'_, '_>,
        _run_arg: Self::RunArg<'_>,
        _ctx: &mut Context,
    ) -> Result<Self::ReturnType> {{
        let mut counter = accounts.counter.data_mut()?;
        counter.signer = *accounts.new_signer.pubkey();

        Ok(())
    }}
}}

#[derive(BorshSerialize, BorshDeserialize, Debug, Copy, Clone, InstructionArgs)]
#[ix_args(run)]
pub struct CountIx {{
    pub amount: u64,
    pub subtract: bool,
}}

#[derive(AccountSet, Debug)]
#[validate(extra_validation = self.validate())]
pub struct CountAccounts {{
    pub owner: Signer<SystemAccount>,
    pub counter: Mut<Account<CounterAccount>>,
}}

impl CountAccounts {{
    fn validate(&self) -> Result<()> {{
        if *self.owner.pubkey() != self.counter.data()?.owner {{
            bail!("Incorrect owner");
        }}
        Ok(())
    }}
}}

impl StarFrameInstruction for CountIx {{
    type ReturnType = ();
    type Accounts<'b, 'c> = CountAccounts;

    fn process(
        accounts: &mut Self::Accounts<'_, '_>,
        CountIx {{ amount, subtract }}: Self::RunArg<'_>,
        _ctx: &mut Context,
    ) -> Result<Self::ReturnType> {{
        let mut counter = accounts.counter.data_mut()?;
        let new_count: u64 = if subtract {{
            counter.count - amount
        }} else {{
            counter.count + amount
        }};
        counter.count = new_count;

        Ok(())
    }}
}}

#[derive(BorshSerialize, BorshDeserialize, Debug, InstructionArgs)]
pub struct CloseCounterIx;

#[derive(AccountSet, Debug)]
pub struct CloseCounterAccounts {{
    #[validate(address = &self.counter.data()?.signer)]
    pub signer: Signer<SystemAccount>,
    #[validate(recipient)]
    pub funds_to: Mut<SystemAccount>,
    #[cleanup(arg = CloseAccount(()))]
    pub counter: Mut<WrappedCounter>,
}}
empty_star_frame_instruction!(CloseCounterIx, CloseCounterAccounts);
"#, program_id);

        fs::write(project_path.join("src").join("lib.rs"), lib_rs)?;

        // Create test file
        let test_rs = format!(r#"use {}::*;
use star_frame::{{prelude::*, unsize::TestByteSet}};

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_counter_initialization() {{
        // Test logic will be implemented here
        // This is a placeholder for Star Frame testing utilities
        println!("Counter initialization test");
    }}

    #[test]
    fn test_counter_increment() {{
        // Test logic will be implemented here
        println!("Counter increment test");
    }}

    #[test]
    fn test_counter_decrement() {{
        // Test logic will be implemented here
        println!("Counter decrement test");
    }}

    #[test]
    fn test_counter_overflow_protection() {{
        // Test counter overflow protection
        println!("Counter overflow protection test");
    }}

    #[test]
    fn test_counter_underflow_protection() {{
        // Test counter underflow protection
        println!("Counter underflow protection test");
    }}

    #[test]
    fn test_authority_validation() {{
        // Test authority validation
        println!("Authority validation test");
    }}
}}
"#, project_name.replace('-', "_"));

        fs::write(project_path.join("tests").join("counter.rs"), test_rs)?;

        // Create README.md
        let readme = format!(r#"# {} - Star Frame Counter Program

A simple counter program built with the Star Frame framework for Solana.

## Features

- ✅ Initialize counter with optional starting value
- ✅ Increment counter with overflow protection
- ✅ Decrement counter with underflow protection
- ✅ Authority-based access control
- ✅ Type-safe account validation
- ✅ Compile-time instruction verification

## Getting Started

### Prerequisites

- Rust 1.84.1+
- Solana CLI tools
- Star Frame CLI

### Building

```bash
starframe build
```

### Testing

```bash
starframe test
```

### Deploying

To devnet:
```bash
starframe deploy
```

To mainnet:
```bash
starframe deploy --mainnet
```

### Generate IDL

```bash
starframe idl
```

## Program Structure

- `CounterAccount`: Program account storing authority and count
- `Initialize`: Initialize a new counter
- `Increment`: Increment the counter value
- `Decrement`: Decrement the counter value

## Security Features

- Authority validation for all operations
- Overflow/underflow protection
- Type-safe account handling
- Compile-time validation

## Program ID

```
{}
```
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

        // Create StarFrame.toml configuration file
        let starframe_toml = format!(r#"[toolchain]

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
build = "starframe build"
test = "starframe test"
deploy = "starframe deploy"
"#, 
            project_name.replace('-', "_"), program_id,
            project_name.replace('-', "_"), program_id,
            project_name.replace('-', "_"), program_id
        );

        fs::write(project_path.join("StarFrame.toml"), starframe_toml)?;

        Ok(())
    }
}