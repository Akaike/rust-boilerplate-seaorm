# Axum Boilerplate with SeaORM

This repository provides a basic boilerplate for building Rust applications using [SeaORM](https://www.sea-ql.org/SeaORM/), an async & dynamic ORM for Rust.

## Stack
- **Axum**: Web framework for building HTTP services.
- **SeaORM**: Async & dynamic ORM for interacting with databases.
- **Validator**: For request data validation.
- **Tower**: Middleware framework for composing asynchronous service chains.
- **Anyhow**: Simplified error handling.

## Features
- **Sample Server**: A basic server setup demonstrating the integration of SeaORM with a RESTful API.

## Project Structure
- `entity/`: Contains the entity definitions automatically generated by SeaORM.
- `migration/`: Contains migration scripts for database schema evolution.
- `server/`: Contains the server implementation, including routes and handlers.
  - `src/dtos/todo.rs`: Data Transfer Objects for the Todo entity.
  - `src/validators/todo.rs`: Custom validation for Todo requests.

## Getting Started

### Prerequisites
- Rust (latest stable version recommended)
- Cargo (Rust’s package manager)

### Installation

#### Clone the Repository:
```bash
git clone https://github.com/Akaike/rust-boilerplate-seaorm
cd rust-boilerplate-seaorm
```

#### Running Migrations:
```bash
cargo run -- generate MIGRATION_NAME
```

#### Running the Server:
Start the server:
```bash
cargo run
```

## Using SeaORM to Create New Entities
To create new entities, follow these steps:
1. Define your database schema in the migration scripts within the `migration/` folder.
2. Run the migration script using:
   ```bash
   cargo run --bin migration
   ```
3. Use the SeaORM cli to generate new entities by running:
   ```bash
   sea-orm-cli generate entity -o src/entity
   ```
   This command generates new entity files based on your database schema.

Refer to the [SeaORM documentation]([SeaORM Documentation](https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/)) for additional details on customizing entities and other advanced features.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.
