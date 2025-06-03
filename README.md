# Hello API

A simple REST API built with Rust and Actix Web framework.

## Features

- GET `/` - Returns "Hello world!" message
- POST `/echo` - Echoes back the request body
- GET `/hey` - Returns "Hey there!" message

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Getting Started

1. Clone the repository:
```bash
git clone <repository-url>
cd hello-api
```

2. Run the server:
```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`

## API Endpoints

### GET /
Returns a hello world message.

### POST /echo
Echoes back whatever is sent in the request body.

Example:
```bash
curl -X POST -H "Content-Type: text/plain" -d "Hello Server!" http://127.0.0.1:8080/echo
```

### GET /hey
Returns a greeting message.

Example:
```bash
curl http://127.0.0.1:8080/hey
```

## Project Structure
```
src/
├── main.rs              # Main application entry point
└── controllers/         # API route handlers
    ├── mod.rs          # Module declarations
    └── main_controller.rs  # Main controller logic
```

## Dependencies

- actix-web: Web framework for Rust