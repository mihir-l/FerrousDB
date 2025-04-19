# FerrousDB
A Key-Value Database written in Rust

## Overview
FerrousDB is a lightweight, in-memory key-value database implemented in pure Rust without any external dependencies. 

## Features
- **In-Memory Storage**: Data is stored in memory for fast access.
- **Thread-Safe**: Uses Rust's `std::sync::Mutex` for safe concurrent access.
- **Custom Protocol**: Implements a basic protocol for client-server communication.
- **No External Dependencies**: Built entirely using Rust's standard library.

## How It Works
1. The server listens for incoming TCP connections.
2. Clients send requests using a custom protocol to perform operations like `GET`, `SET`, and `DELETE`.
3. The server processes the requests and sends back appropriate responses.

## Usage
### Running the Server
To start the FerrousDB server, run the following command:
```bash
cargo run