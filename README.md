# Brainiary

A Rust-based REST API application built with Actix-web that provides user authentication, resume generation, and management capabilities.

## Features

- **User Authentication**: JWT-based authentication with bcrypt password hashing
- **Resume Generation**: Generate and manage professional resumes
- **Role-Based Access Control**: User roles for permission management
- **PostgreSQL Database**: Persistent data storage with Diesel ORM
- **CORS Support**: Cross-origin resource sharing enabled for frontend integration
- **Comprehensive Logging**: Built-in logging with env_logger

## Tech Stack

- **Framework**: Actix-web 4
- **Database**: PostgreSQL with Diesel ORM
- **Authentication**: JWT (jsonwebtoken) + bcrypt
- **Runtime**: Tokio async runtime
- **Serialization**: Serde JSON
- **HTTP Client**: Reqwest

## Prerequisites

- Rust 1.70 or later
- PostgreSQL 12 or later
- Cargo (comes with Rust)

## Environment Setup

### 1. Install Dependencies

Make sure you have Rust and PostgreSQL installed:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify Rust installation
rustc --version
cargo --version
```

### 2. Set Up PostgreSQL

```bash
# Create a new database for the project
createdb brainiary_db

# Verify the database was created
psql -l | grep brainiary_db
```

### 3. Configure Environment Variables

Copy the environment sample and update it with your configuration:

```bash
cp env_sample .env
```

Edit `.env` with your settings:

```plaintext
# Database configuration
DATABASE_URL=postgres://username:password@localhost/brainiary_db

# OpenAI API (optional, for resume generation features)
OPENAPI_API_KEY=your_openai_api_key_here

# Logging level (optional)
RUST_LOG=info
```

### 4. Run Database Migrations

```bash
# Install Diesel CLI if you haven't already
cargo install diesel_cli --no-default-features --features postgres

# Run all pending migrations
diesel migration run
```

## Building and Running

### Development

```bash
# Build the project
cargo build

# Run the development server
cargo run

# The server will start at http://localhost:8000
```

### Production

```bash
# Build optimized release binary
cargo build --release

# Run the release binary
./target/release/Brainiary
```

### Running Tests

```bash
cargo test
```

### Watch Mode (requires cargo-watch)

```bash
# Install cargo-watch
cargo install cargo-watch

# Run in watch mode for development
cargo watch -x run
```

## Project Structure

```
src/
├── main.rs              # Application entry point
├── schema.rs            # Diesel schema definitions
├── auth/                # Authentication module
│   ├── handler.rs       # Auth route handlers
│   ├── model.rs         # Auth data models
│   └── mod.rs           # Module definition
├── resume/              # Resume module
│   ├── handler.rs       # Resume route handlers
│   ├── model.rs         # Resume data models
│   ├── generator.rs     # Resume generation logic
│   └── mod.rs           # Module definition
└── db/                  # Database module
    ├── db.rs            # Database connection setup
    └── mod.rs           # Module definition

migrations/             # Database migrations
├── 00000000000000_diesel_initial_setup/
├── 2025-05-26-054820_create_users/
└── 2025-06-09-062425_add_role_to_users/
```

## API Endpoints

### Authentication Routes
- `POST /auth/login` - User login
- `POST /auth/register` - User registration
- `POST /auth/logout` - User logout

### Resume Routes
- `GET /resume/:id` - Get a specific resume
- `POST /resume` - Create a new resume
- `PUT /resume/:id` - Update a resume
- `DELETE /resume/:id` - Delete a resume

## Database Migrations

The project includes Diesel migrations for setting up the database schema:

```bash
# Create a new migration
diesel migration generate migration_name

# Run all pending migrations
diesel migration run

# Revert the last migration
diesel migration revert
```

## Logging

The application uses `env_logger` for logging. Control the log level with the `RUST_LOG` environment variable:

```bash
# Set log level
export RUST_LOG=info    # info, debug, warn, error
cargo run
```

## CORS Configuration

The application is configured to accept CORS requests from any origin with the following allowed methods:
- GET
- POST
- PUT
- DELETE
- OPTIONS

Modify the CORS configuration in `src/main.rs` for production use.

## Troubleshooting

### Database Connection Issues

```bash
# Test PostgreSQL connection
psql postgres://username:password@localhost/brainiary_db

# Check if PostgreSQL is running (macOS)
brew services list | grep postgres
```

### Migration Errors

```bash
# Check migration status
diesel migration list

# Reset database (careful - deletes all data)
diesel database reset
```

### Build Issues

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Build with verbose output
cargo build --verbose
```

## Contributing

1. Create a new branch for your feature
2. Make your changes
3. Test thoroughly
4. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues, questions, or suggestions, please create an issue in the repository.
