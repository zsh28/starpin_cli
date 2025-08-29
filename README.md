<div align="center">
  <img src="assets/logo.png" alt="Starpin Logo" width="200" height="200">
  
  # Starpin CLI

  [![Crates.io](https://img.shields.io/crates/v/starpin.svg)](https://crates.io/crates/starpin)
  [![Downloads](https://img.shields.io/crates/d/starpin.svg)](https://crates.io/crates/starpin)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

  A command-line interface for creating and managing [Star Frame](https://github.com/staratlasmeta/star_frame) Solana programs.
</div>

## Installation

### Using Cargo (Recommended)

Install directly from [crates.io](https://crates.io/crates/starpin):

```bash
cargo install starpin
```

### From Source

```bash
git clone https://github.com/zsh28/starpin_cli
cd starpin_cli
cargo build --release
# Add target/release/starpin to your PATH
```

## Usage

### Initialize a New Project

Create a new Star Frame project using the counter template:

```bash
starpin init my_project
```

Create a project with a specific template:

```bash
starpin init my_project --template counter
```

Create a project in a specific directory:

```bash
starpin init my_project --path ./projects
```

Available templates:
- `counter` - Full-featured counter with create, update, increment, decrement, and close operations
- `simple_counter` - Basic counter with just initialize and increment (perfect for learning)
- `marketplace` - Advanced order book marketplace with SPL token integration

### Build Your Program

Build for localnet (default):

```bash
starpin build
```

Build for specific network:

```bash
starpin build --network devnet
starpin build --network mainnet
```

Skip automatic IDL generation:

```bash
starpin build --skip-idl
```

### Test Your Program

Run all tests (localnet):

```bash
starpin test
```

Run tests against specific network:

```bash
starpin test --network devnet
```

Run tests with a filter:

```bash
starpin test --filter counter
```

### Deploy Your Program

Deploy to devnet (default):

```bash
starpin deploy
```

Deploy to specific network:

```bash
starpin deploy --network localnet
starpin deploy --network mainnet
```

Upgrade an existing program:

```bash
starpin deploy --network devnet --program-id <PROGRAM_ID>
```

### Generate IDL

Generate IDL files for client libraries:

```bash
starpin idl
```

Generate IDL to a specific directory:

```bash
starpin idl --output ./my-idl
```

### Check Network Configuration

View current Solana network settings:

```bash
starpin network
```

### Generate New Program Keys

Generate a new random program ID (like `anchor keys list`):

```bash
starpin keys
```

Generate keys for a specific program:

```bash
starpin keys --program my_program
```

### Sync Program IDs

Synchronize program IDs between Starpin.toml and lib.rs (like `anchor keys sync`):

```bash
starpin sync
```

Use program ID from lib.rs as source of truth:

```bash
starpin sync --from-lib
```

## ✨ Features

- 🚀 **Project template generation** - Bootstrap projects with production-ready templates
- 🔒 **Type-safe Star Frame programs** - Compile-time safety for Solana development
- ⚡ **Integrated build system** - Automatic IDL generation and optimized builds
- 🌐 **Multi-network support** - Seamless deployment to localnet/devnet/mainnet
- 🧪 **Testing utilities** - Network-aware testing with mollusk-svm integration
- 📦 **Smart dependency management** - Automatic latest version fetching
- ⚙️ **Configuration management** - Professional Starpin.toml configuration
- 🔑 **Program key management** - Generate and sync program IDs like Anchor
- 📋 **IDL generation** - Generate client libraries and type definitions
- 🔄 **Automatic synchronization** - Keep program IDs in sync across files

## 🤔 Why Starpin?

Starpin is the **official CLI** for [Star Frame](https://github.com/staratlasmeta/star_frame), bringing the power of type-safe Solana development to your fingertips:

- **🎯 Focus on logic, not boilerplate** - Star Frame eliminates repetitive Solana program code
- **🛡️ Type safety** - Catch errors at compile-time with Rust's type system
- **⚡ Faster development** - Modern templates and tooling for rapid prototyping
- **🔧 Production ready** - Professional patterns used in real-world applications
- **📚 Great DX** - Anchor-like commands with improved ergonomics

## 🚀 Quick Start

Get up and running in less than 2 minutes:

```bash
# Install starpin
cargo install starpin

# Create a new counter project
starpin init my-counter --template counter

# Navigate to your project
cd my-counter

# Build and test
starpin build
starpin test

# Deploy to devnet
starpin deploy --network devnet
```

## Project Structure

Generated projects include:

```
my_project/
├── Cargo.toml           # Rust manifest with Star Frame dependencies
├── Starpin.toml       # Network and deployment configuration
├── src/
│   ├── lib.rs          # Main program logic
│   └── main.rs         # Binary entry point
├── tests/
│   └── counter.rs      # Test files
├── README.md           # Project documentation
└── .gitignore          # Git ignore patterns
```

## Templates Overview

### Counter Template (Advanced)
- **Create Counter**: Initialize with owner, signer, and optional starting value
- **Update Signer**: Change the authorized signer for the counter
- **Count**: Add or subtract amounts with validation
- **Close Counter**: Clean up and recover rent
- **Full validation**: Advanced authority and state management

### Simple Counter Template (Beginner)
- **Initialize**: Create a counter with optional starting value
- **Increment**: Simple increment by 1 operation
- **Minimal complexity**: Perfect for learning Star Frame basics
- **Clean code**: Easy to understand and extend

### Marketplace Template (Professional)
- **Market Creation**: Initialize markets for any SPL token pair
- **Order Placement**: Place buy/sell orders with price/quantity
- **Order Cancellation**: Cancel single or multiple orders
- **Order Matching**: Automatic bid/ask matching engine
- **SPL Integration**: Full token transfer and escrow system
- **Advanced Features**: Fill-or-kill orders, maker info tracking
- **Production Ready**: Comprehensive validation and error handling

## Development

### Prerequisites

- Rust 1.84.1+
- Solana CLI tools
- cargo-build-sbf (for Solana program compilation)

### Building the CLI

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Adding New Templates

1. Create a new module in `src/templates/`
2. Implement the `Template` trait
3. Add the template to the match statement in `src/commands/init.rs`

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Links

- 🌟 [Star Frame Framework](https://github.com/staratlasmeta/star_frame) - The framework this CLI supports
- 📦 [Crates.io Package](https://crates.io/crates/starpin)
- 📚 [Documentation](https://docs.rs/starpin)
- 🐛 [Issues & Bug Reports](https://github.com/zsh28/starpin_cli/issues)
- 💬 [Discussions](https://github.com/zsh28/starpin_cli/discussions)

## Support

For issues and questions:
- Open an issue on [GitHub](https://github.com/zsh28/starpin_cli/issues)
- Check the [Star Frame framework](https://github.com/staratlasmeta/star_frame)
- Join the Solana developer community