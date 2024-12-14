## Overview

`indexer-rs` is a robust, production-ready service built in Rust that monitors and indexes events from EVM-compatible blockchain networks. The service is designed for high reliability and performance, with features tailored for blockchain data indexing.

### Key Features

- **Multi-Chain Support**
  - Compatible with any EVM-based blockchain (Ethereum, Polygon, BSC, etc.)
  - Configurable chain-specific parameters

- **Smart Contract Event Monitoring**
  - Concurrent monitoring of multiple contract addresses
  - Support for both indexed and non-indexed events
  - Configurable event filtering
  - Real-time event processing

- **Performance Optimizations**
  - Intelligent rate limiting based on chain block times
  - Batch processing of events
  - Connection pooling for database operations
  - Efficient memory management

- **Data Persistence**
  - PostgreSQL integration for reliable data storage
  - Structured event data indexing
  - Transaction-safe database operations
  - Automatic retry mechanisms

- **Operational Features**
  - Graceful error handling and recovery
  - TODO: Comprehensive logging and monitoring

- **Developer-Friendly**
  - Clear configuration through environment variables
  - Docker support for easy deployment
  - Extensive documentation
  - Modular architecture for easy extensions

### Use Cases

- DeFi protocol analytics
- NFT marketplace indexing
- On-chain activity monitoring
- Cross-chain data aggregation
- Smart contract event auditing
- Blockchain data analysis

## Project Architecture

### Overview

The project follows a modular, event-driven architecture designed for scalability and maintainability. It consists of three main components that work together to provide a robust blockchain indexing solution.

### Core Components

#### 1. Database Library (`libs/indexer-db`)
- **Purpose**: Centralizes database entities and operations
- **Location**: `libs/indexer-db/`
- **Key Features**:
  - Type-safe database schema definitions
  - Reusable database interfaces
  - Connection pooling management
  - Migration utilities
  - Transaction handling

#### 2. Event Listener (`listener`)
- **Purpose**: Monitors blockchain events in real-time
- **Location**: `listener/`
- **Key Features**:
  - Connects to blockchain nodes
  - Filters all events
  - Stores unprocessed logs in database
  - Manages connection retries

#### 3. Event Processor (`processor`)
- **Purpose**: Provides a framework for processing blockchain events with custom business logic
- **Location**: `processor/`
- **Key Features**:
  - Abstract processor traits for easy implementation
  - Automated event handling pipeline
  - Built-in error handling and retry mechanisms
  - Configurable batch processing

### Benefits of Modular Design

1. **Separation of Concerns**
   - Each component has a single, well-defined responsibility
   - Easier to maintain and test individual components
   - Reduces complexity in each module

2. **Scalability**
   - Components can be scaled independently
   - Horizontal scaling of processors for high-throughput
   - Multiple listeners can work with different chains

3. **Reliability**
   - Failure in one component doesn't affect others
   - Easy to implement retry mechanisms
   - Better error isolation and handling

4. **Development Efficiency**
   - Teams can work on different components simultaneously
   - Clear interfaces between components
   - Reusable code across different implementations

5. **Flexibility**
   - Easy to add support for new chains
   - Simple to implement custom processing logic
   - Pluggable architecture for different use cases

## Docker Setup

### Prerequisites

- Docker and Docker Compose installed
- Git (for cloning the repository)

### Development Environment

1. **Clone and Setup**
   ```bash
   git clone git@github.com:abhinavmsra/indexer-rs.git
   cd indexer-rs
   ```

2. **Configure Environment Variables**
    - Copy the `.env.example` file to `.env` and configure the variables.
    ```bash
    cp .env.example .env
    ```

    - You can also set the variables in the docker compose file.

### Environment Variables Reference

### Database Configuration
| Variable    | Description                     | Example Value           | Required |
|-------------|---------------------------------|-----------------------|-----------|
| PGHOST      | PostgreSQL host                 | `db`                 | Yes       |
| PGPORT      | PostgreSQL port                 | `5432`               | Yes       |
| PGUSER      | PostgreSQL user                 | `app`                | Yes       |
| PGDATABASE  | PostgreSQL database name        | `indexer_development`| Yes       |
| PGAPPNAME   | Application name in PostgreSQL  | `listener`           | Yes       |

### Listener Configuration
| Variable           | Description                          | Example Value                                  | Required |
|--------------------|--------------------------------------|------------------------------------------------|-----------|
| CHAIN_ID          | Blockchain network identifier        | `84532` (Base Sepolia)                         | Yes       |
| CONTRACT_ADDRESSES | Smart contract addresses to monitor  | `4752ba5DBc23f44D87826276BF6Fd6b1C372aD24`    | Yes       |
| RPC_URL           | Blockchain node RPC endpoint         | `https://base-sepolia.g.alchemy.com/v2/XXXXX`  | Yes       |

### Processor Configuration
| Variable            | Description                               | Example Value                                           | Required |
|---------------------|-------------------------------------------|--------------------------------------------------------|-----------|
| ARTIFACTS_BASE_PATH | Directory path for contract ABI files     | `processor/artifacts/abi`                              | Yes       |
| CONTRACTS          | Contract name and address mapping         | `uniswap_v3_factory:4752ba5DBc23f44D87826276BF6Fd6b1C372aD24` | Yes       |

### Notes:
- Multiple contract addresses can be specified as comma-separated values
- CONTRACTS format: `contract_name:contract_address,contract_name:contract_address`
- All paths are relative to the project root
- Make sure to replace placeholder values (XXXXX) with actual credentials

## Development Setup Options

You have two options for setting up your development environment:

### Option 1: Local Development with Docker Compose

1. **Start the Services**
   ```bash
   docker compose up -d
   ```
   This will start:
   - PostgreSQL database
   - Development container with all tools. If you are not using the dev container, you can run remove the `dev` service from the docker compose file.

2. **Run Commands Locally**
   ```bash
   # Build the project
   cargo build

   # Run tests
   cargo test

   # Start the application
   cargo run -p {listener/processor}
   ```

### Option 2: VS Code Dev Containers (Recommended)

This option provides a fully configured development environment with IDE integration.

1. **Prerequisites**
   - VS Code installed
   - Docker Desktop running
   - [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension installed

2. **Open in Dev Container**
   - Open project in VS Code
   - Click the blue button in bottom-left corner
   - Select "Reopen in Container"

   OR
   ```bash
   code .
   # Use Command Palette (Ctrl/Cmd + Shift + P)
   # Select "Dev Containers: Reopen in Container"
   ```

## Database Migrations

Before running the application, you need to set up the database schema by running migrations.

### Running Migrations

1. **Navigate to the database library**
   ```bash
   cd libs/indexer-db
   ```

2. **Run migrations**
   ```bash
   sqlx migrate run
   ```

### Verify Migration Status

```bash
# Check current migration status
sqlx migrate info
```

### Common Migration Tasks

```bash
# Create a new migration
sqlx migrate add <migration_name>

# Revert last migration
sqlx migrate revert

# Reset database (revert all migrations)
sqlx migrate revert --all
```

After successful migration, you can proceed with running the application components.

## Running the Applications

Since this is a workspace project with multiple components, you'll need to specify which component to run.

### Running the Listener

```bash
# Build the listener
cargo build -p listener

# Run the listener
cargo run -p listener
```

### Running the Processor

```bash
# Build the processor
cargo build -p processor

# Run the processor
cargo run -p processor
```

### Development Mode (with hot reloading)

```bash
# Watch and run listener
cargo watch -x 'run -p listener'

# Watch and run processor
cargo watch -x 'run -p processor'
```

### Running Tests

```bash
# Test all workspace members
cargo test --workspace

# Test specific package
cargo test -p listener
cargo test -p processor
cargo test -p indexer-db

# Run tests with logging
RUST_LOG=debug cargo test -p listener
```

### Common Development Tasks

```bash
# Check all workspace members
cargo check --workspace

# Format all code
cargo fmt --all

# Run clippy on all workspace members
cargo clippy --workspace

# Build all packages in release mode
cargo build --workspace --release
```
