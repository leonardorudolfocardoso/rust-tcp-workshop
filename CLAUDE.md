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

Standard library (used in the thread-based implementation):

- [`std::net`](https://doc.rust-lang.org/std/net/index.html) — `TcpListener`, `TcpStream`, `SocketAddr`
- [`std::io`](https://doc.rust-lang.org/std/io/index.html) — `Read`, `Write`, `BufReader`
- [`std::thread`](https://doc.rust-lang.org/std/thread/index.html) — spawning threads to handle concurrent connections
- [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html) — channel for sending jobs to worker threads
- [`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) — shared ownership across threads
- [`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) — mutual exclusion for shared receiver

Tokio (async implementation):

- [`tokio::net::TcpListener`](https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html) — async TCP listener
- [`tokio::net::TcpStream`](https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html) — async TCP stream, `split()` for separate read/write halves
- [`tokio::io::AsyncBufReadExt`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncBufReadExt.html) — `read_line` on async `BufReader`
- [`tokio::io::AsyncWriteExt`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncWriteExt.html) — `write_all`, `flush` on async `BufWriter`

## Project

This is a Rust project (`edition = "2024"`) intended to implement a TCP server. Currently at initial scaffold — `src/main.rs` only contains the default `Hello, world!` entry point with no dependencies yet.
