# rust-tcp-workshop

A hands-on workshop project for learning TCP server implementation in Rust. The project builds two versions of the same echo server — one using OS threads and one using async/await with Tokio — to explore the trade-offs between the two concurrency models.

## Implementations

### Sync server (`src/bin/sync_server.rs`)

A thread-pool-based TCP server using only the standard library.

- `TcpListener` accepts connections in a loop
- A fixed-size `ThreadPool` distributes work via an `mpsc` channel
- Workers share the channel receiver behind an `Arc<Mutex<_>>`
- Each connection is handled synchronously with `BufReader`/`Write`

Run with:

```bash
cargo run --bin sync_server
```

### Async server (`src/bin/async_server.rs`)

An async TCP server using Tokio.

- `tokio::net::TcpListener` accepts connections
- Each connection is spawned as an independent task with `tokio::spawn`
- `TcpStream::split()` gives separate reader/writer halves without cloning
- I/O uses `AsyncBufReadExt` and `AsyncWriteExt`

Run with:

```bash
cargo run --bin async_server
```

Both servers listen on `127.0.0.1:7878` and echo back each line prefixed with `I read and processed the message`.

## Try it out

```bash
# In one terminal, start a server
cargo run --bin async_server

# In another terminal, connect and send messages
nc 127.0.0.1 7878
hello
# → I read and processed the message hello
```

## Commands

```bash
cargo build    # Build
cargo test     # Run tests
cargo clippy   # Lint
cargo fmt      # Format
```
