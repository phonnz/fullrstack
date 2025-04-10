# Fullrstack

High-performance WebSocket system built with Rust and Leptos.

## Prerequisites

- Rust (latest stable)
- Node.js (for frontend development tools)
- Docker and Docker Compose
- PostgreSQL (if running locally)

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/yourusername/fullrstack.git
cd fullrstack
```

2. Build and run with Docker:
```bash
docker-compose up --build
```

3. Visit http://localhost:3000 for the dashboard 

## Runing Lovcally without Docker
- Create the database
```
psql -U postgres -c "CREATE USER fullrstack WITH PASSWORD 'fullrstack';"
psql -U postgres -c "CREATE DATABASE fullrstack OWNER fullrstack;"
```
- Create a .env file in the project root
```
DATABASE_URL=postgres://fullrstack:fullrstack@localhost:5432/fullrstack
RUST_LOG=debug
SERVER_URL=ws://localhost:8080
```
- Migrate 
`cargo run -p fullrstack-server -- migrate`

- Start the server
`cargo run -p fullrstack-server`

## Test the server running a client example
 cargo run -p fullrstack-examples --bin test_client
