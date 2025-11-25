# Quick Reference Guide

## 🚀 Quick Start Commands

### Run Default Welcome Program
```bash
cargo run
```

### Run Individual Examples

```bash
# Example 01: Async Basics
cargo run --bin 01_tokio_async_await_basics

# Example 02: Concurrent Tasks
cargo run --bin 02_tokio_spawn_concurrent_tasks

# Example 03: MPSC Channels
cargo run --bin 03_tokio_mpsc_channel_communication

# Example 04: Async File Read
cargo run --bin 04_tokio_file_read_async

# Example 05: CSV Basic
cargo run --bin 05_csv_reader_basic

# Example 06: CSV with Serde
cargo run --bin 06_csv_deserialize_with_serde

# Example 07: Redis Client (requires mini-redis-server)
cargo run --bin 07_mini_redis_client_operations

# Example 08: Redis Actor Pattern (requires mini-redis-server)
cargo run --bin 08_mini_redis_actor_pattern
```

## 🔧 Development Commands

```bash
# Build all examples
cargo build

# Build in release mode (optimized)
cargo build --release

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# View dependency tree
cargo tree
```

## 📦 Redis Server Commands

```bash
# Install mini-redis (one-time)
cargo install mini-redis

# Start mini-redis server
mini-redis-server

# Start on custom port
mini-redis-server --port 6380
```

## 📊 File Mapping

| Old Name | New Name |
|----------|----------|
| `test_async_basics.rs` | `01_tokio_async_await_basics.rs` |
| `test_tokio_spawn.rs` | `02_tokio_spawn_concurrent_tasks.rs` |
| `test_channel_mpsc.rs` | `03_tokio_mpsc_channel_communication.rs` |
| `test_read_txt_file.rs` | `04_tokio_file_read_async.rs` |
| `test_read_csv_file.rs` | `05_csv_reader_basic.rs` |
| `test_read_csv_deserialize.rs` | `06_csv_deserialize_with_serde.rs` |
| `test_miniredis_tokio.rs` | `07_mini_redis_client_operations.rs` |
| `test_mini_redis_channel.rs` | `08_mini_redis_actor_pattern.rs` |

## 🎯 Quick Tips

1. **Start Sequential**: Begin with example 01 and work your way up
2. **Read Comments**: Each file has detailed explanations
3. **Modify & Experiment**: Change values and see what happens
4. **Check Errors**: Rust's compiler messages are very helpful
5. **Use README**: Full documentation in README.md

## 🐛 Troubleshooting

### Redis Connection Error
```
Error: Connection refused (os error 10061)
```
**Solution**: Start mini-redis-server first: `mini-redis-server`

### File Not Found
```
Error: No such file or directory (os error 2)
```
**Solution**: Ensure you're in the project root directory and `data/` folder exists

### Build Errors
```
error: could not compile `Rust101`
```
**Solution**: Run `cargo clean` then `cargo build`

