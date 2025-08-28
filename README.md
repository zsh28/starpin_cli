# Star Frame CLI

A command-line interface for creating and managing Star Frame Solana programs.

## Installation

### From Source

```bash
git clone <repository-url>
cd star_frame_cli
cargo build --release
# Add target/release/starframe to your PATH
```

### Using Cargo

```bash
cargo install starframe
```

## Usage

### Initialize a New Project

Create a new Star Frame project using the counter template:

```bash
starframe init my_project
```

Create a project with a specific template:

```bash
starframe init my_project --template counter
```

Create a project in a specific directory:

```bash
starframe init my_project --path ./projects
```

Available templates:
- `counter` - Full-featured counter with create, update, increment, decrement, and close operations
- `simple_counter` - Basic counter with just initialize and increment (perfect for learning)
- `marketplace` - Advanced order book marketplace with SPL token integration

### Build Your Program

Build for localnet (default):

```bash
starframe build
```

Build for specific network:

```bash
starframe build --network devnet
starframe build --network mainnet
```

Skip automatic IDL generation:

```bash
starframe build --skip-idl
```

### Test Your Program

Run all tests (localnet):

```bash
starframe test
```

Run tests against specific network:

```bash
starframe test --network devnet
```

Run tests with a filter:

```bash
starframe test --filter counter
```

### Deploy Your Program

Deploy to devnet (default):

```bash
starframe deploy
```

Deploy to specific network:

```bash
starframe deploy --network localnet
starframe deploy --network mainnet
```

Upgrade an existing program:

```bash
starframe deploy --network devnet --program-id <PROGRAM_ID>
```

### Generate IDL

Generate IDL files for client libraries:

```bash
starframe idl
```

Generate IDL to a specific directory:

```bash
starframe idl --output ./my-idl
```

### Check Network Configuration

View current Solana network settings:

```bash
starframe network
```

### Generate New Program Keys

Generate a new random program ID (like `anchor keys list`):

```bash
starframe keys
```

Generate keys for a specific program:

```bash
starframe keys --program my_program
```

### Sync Program IDs

Synchronize program IDs between StarFrame.toml and lib.rs (like `anchor keys sync`):

```bash
starframe sync
```

Use program ID from lib.rs as source of truth:

```bash
starframe sync --from-lib
```

## Features

- ✅ Project template generation
- ✅ Type-safe Star Frame program templates
- ✅ Integrated build system with automatic IDL generation
- ✅ Multi-network support (localnet/devnet/mainnet)
- ✅ Testing utilities with network configuration
- ✅ Network-aware deployment
- ✅ Professional network management
- ✅ Configuration files (StarFrame.toml)
- ✅ Program key management (generate/sync)
- ✅ Automatic program ID synchronization

## Project Structure

Generated projects include:

```
my_project/
├── Cargo.toml           # Rust manifest with Star Frame dependencies
├── StarFrame.toml       # Network and deployment configuration
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

## Support

For issues and questions:
- Open an issue on GitHub
- Check the Star Frame documentation
- Join the Solana developer community