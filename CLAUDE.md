# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build          # Build the project
cargo run            # Run the server
cargo test           # Run all tests
cargo test <name>    # Run a single test by name
cargo clippy         # Lint
cargo fmt            # Format code
```

## Collaboration Style

Act as a tutorial guide. When implementing features, explain *why* each piece works the way it does — cover Rust concepts (ownership, traits, stdlib types) as they appear. Walk through changes step by step rather than dumping complete implementations. Encourage the user to try things before revealing the answer.

## Documentation

Standard library modules used in this project:

- [`std::net`](https://doc.rust-lang.org/std/net/index.html) — `TcpListener`, `TcpStream`, `SocketAddr`
- [`std::io`](https://doc.rust-lang.org/std/io/index.html) — `Read`, `Write`, `BufReader`
- [`std::thread`](https://doc.rust-lang.org/std/thread/index.html) — spawning threads to handle concurrent connections

> Update this section as new crates are added to `Cargo.toml`.

## Project

This is a Rust project (`edition = "2024"`) intended to implement a TCP server. Currently at initial scaffold — `src/main.rs` only contains the default `Hello, world!` entry point with no dependencies yet.
