# Rust101 - Learning Rust with Tokio and Async Programming

A comprehensive collection of Rust examples focusing on asynchronous programming with Tokio, file I/O, CSV processing, and Redis operations.

## 📚 Project Overview

This project contains 8 progressive examples demonstrating key Rust concepts, from basic async/await to advanced actor patterns with Redis. Each example is numbered sequentially for easy learning progression.

## 🚀 Prerequisites

- Rust 1.70+ (edition 2024)
- Mini-Redis server (for examples 07 and 08)

## 📦 Dependencies

```toml
tokio = { version = "1", features = ["full"] }
mini-redis = "0.4.1"
bytes = "1.11.0"
csv = "1.3.1"
serde = { version = "1.0", features = ["derive"] }
```

## 📖 Learning Path

### Complete File Reference Table

| # | File Name | Description | Key Concepts | Dependencies |
|---|-----------|-------------|--------------|--------------|
| 01 | `01_tokio_async_await_basics.rs` | **Introduction to async/await syntax** - Demonstrates the fundamentals of asynchronous programming with Tokio including async functions, delays, concurrent task execution using `tokio::join!`, and basic channel communication. Three comprehensive chapters covering async basics, concurrent tasks, and async communication patterns. | `async/await`, `tokio::time::sleep`, `tokio::spawn`, `tokio::join!`, `mpsc channels`, async functions with return values | `tokio` |
| 02 | `02_tokio_spawn_concurrent_tasks.rs` | **Spawning concurrent tasks** - Shows how to use `tokio::spawn` to run multiple tasks concurrently. Demonstrates task ordering and waiting with JoinHandle. Simple example with two async functions (`hello` and `world`) that sleep for different durations, illustrating non-blocking execution. | `tokio::spawn`, `JoinHandle`, `async fn`, `tokio::time::sleep`, concurrent execution order | `tokio` |
| 03 | `03_tokio_mpsc_channel_communication.rs` | **Multi-Producer Single-Consumer channels** - Demonstrates inter-task communication using `mpsc` channels. Multiple senders (cloned from original) send messages to a single receiver. Shows how to handle messages from multiple producers concurrently. | `mpsc::channel`, `Sender::clone()`, `Receiver::recv()`, multi-producer pattern, channel buffer size | `tokio` |
| 04 | `04_tokio_file_read_async.rs` | **Async file reading** - Shows how to read files asynchronously using Tokio's file system APIs. Opens and reads a text file using `AsyncReadExt` trait, demonstrating non-blocking I/O operations with proper buffer handling. | `tokio::fs::File`, `AsyncReadExt`, async I/O, buffer management, `String::from_utf8_lossy` | `tokio` |
| 05 | `05_csv_reader_basic.rs` | **Basic CSV reading** - Demonstrates parsing CSV files using the csv crate. Reads records from a CSV file containing population data, showing error handling with `Result` and the iterator pattern for processing records. | `csv::Reader`, CSV parsing, `StringRecord`, error handling with `?`, `Result<T, E>` | `csv` |
| 06 | `06_csv_deserialize_with_serde.rs` | **CSV with Serde deserialization** - Advanced CSV processing that deserializes CSV rows directly into strongly-typed Rust structs. Uses Serde's derive macros for automatic deserialization. Handles optional fields (`Option<T>`) and demonstrates type-safe data processing. | `serde::Deserialize`, `#[derive]` macros, struct deserialization, `Option<T>`, type-safe CSV processing | `csv`, `serde` |
| 07 | `07_mini_redis_client_operations.rs` | **Redis client operations** - Comprehensive guide to using mini-redis client with multiple practical examples: basic SET/GET operations, storing user profiles, message queue simulation, updating values, and handling non-existent keys. Shows async Redis operations with proper error handling. | Redis client connection, `client.set()`, `client.get()`, key-value storage, `Option` handling, async Redis operations | `mini-redis`, `tokio` |
| 08 | `08_mini_redis_actor_pattern.rs` | **Actor pattern with Redis** - Advanced example demonstrating the actor pattern for managing shared Redis connections. Uses `mpsc` for commands and `oneshot` for responses. Multiple tasks send GET/SET commands to a single manager task that owns the Redis connection, preventing race conditions. Production-ready pattern. | Actor pattern, `oneshot::channel`, `mpsc::channel`, command enum, `Responder` type, connection multiplexing, race condition prevention | `mini-redis`, `tokio`, `bytes` |

## 🎯 Learning Objectives by Category

### Async Programming (Files 01-04)
- ✅ Understanding `async`/`await` syntax
- ✅ Creating and spawning concurrent tasks
- ✅ Inter-task communication with channels
- ✅ Async file I/O operations

### Data Processing (Files 05-06)
- ✅ Reading and parsing CSV files
- ✅ Type-safe deserialization with Serde
- ✅ Handling optional values
- ✅ Error propagation patterns

### Redis Operations (Files 07-08)
- ✅ Connecting to Redis servers
- ✅ Basic CRUD operations
- ✅ Actor pattern for resource management
- ✅ Command multiplexing with channels

## 🏃 Running Examples

Each file is a standalone example. To run a specific example:

```bash
# Run example 01 - Async Basics
cargo run --bin 01_tokio_async_await_basics

# Run example 02 - Tokio Spawn
cargo run --bin 02_tokio_spawn_concurrent_tasks

# Run example 03 - MPSC Channels
cargo run --bin 03_tokio_mpsc_channel_communication

# Run example 04 - Async File Read
cargo run --bin 04_tokio_file_read_async

# Run example 05 - Basic CSV
cargo run --bin 05_csv_reader_basic

# Run example 06 - CSV with Serde
cargo run --bin 06_csv_deserialize_with_serde

# Run example 07 - Redis Client (requires redis server)
cargo run --bin 07_mini_redis_client_operations

# Run example 08 - Redis Actor Pattern (requires redis server)
cargo run --bin 08_mini_redis_actor_pattern
```

### For Redis Examples (07, 08)

Start the mini-redis server first:

```bash
# Install mini-redis-server if not already installed
cargo install mini-redis

# Start the server
mini-redis-server
```

Then run the examples in a separate terminal.

## 📁 Project Structure

```
Rust101/
├── Cargo.toml                                    # Project configuration
├── Cargo.lock                                    # Dependency lock file
├── README.md                                     # This file
├── .gitignore                                    # Git ignore rules
├── data/                                         # Data files for examples
│   ├── hello.txt                                # Text file for example 04
│   └── smallpop.csv                             # CSV file for examples 05-06
├── src/
│   ├── main.rs                                  # Project entry point
│   ├── 01_tokio_async_await_basics.rs          # Async fundamentals
│   ├── 02_tokio_spawn_concurrent_tasks.rs      # Task spawning
│   ├── 03_tokio_mpsc_channel_communication.rs  # Channel communication
│   ├── 04_tokio_file_read_async.rs             # Async file operations
│   ├── 05_csv_reader_basic.rs                  # Basic CSV reading
│   ├── 06_csv_deserialize_with_serde.rs        # Advanced CSV with Serde
│   ├── 07_mini_redis_client_operations.rs      # Redis client usage
│   └── 08_mini_redis_actor_pattern.rs          # Redis actor pattern
└── target/                                       # Build artifacts (gitignored)
```

## 🔑 Key Concepts Explained

### Async/Await
```rust
async fn my_function() {
    // Async operations can be awaited
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### Tokio Spawn
```rust
let handle = tokio::spawn(async {
    // This runs concurrently
});
handle.await.unwrap();
```

### MPSC Channels
```rust
let (tx, mut rx) = mpsc::channel(32);
tx.send("message").await.unwrap();
let msg = rx.recv().await;
```

### Actor Pattern
```rust
enum Command {
    Get { key: String, resp: Responder },
    Set { key: String, val: Value, resp: Responder },
}
// Manager task owns the resource
// Other tasks send commands via channels
```

## 🛠️ Common Patterns

### Error Handling
- Use `Result<T, E>` for functions that can fail
- Use `?` operator to propagate errors
- Use `unwrap()` only in examples (use proper error handling in production)

### Async Best Practices
- Always `.await` futures to execute them
- Use `tokio::spawn` for concurrent tasks
- Use channels for inter-task communication
- Avoid sharing mutable state; use message passing instead

### Resource Management
- Use actor pattern for shared resources (like database connections)
- One task owns the resource, others communicate via channels
- Prevents race conditions and simplifies concurrency

## 📚 Additional Resources

- [Tokio Documentation](https://tokio.rs/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [CSV Crate Docs](https://docs.rs/csv/)
- [Serde Documentation](https://serde.rs/)
- [Mini-Redis Tutorial](https://tokio.rs/tokio/tutorial)

## 🎓 Learning Progression

1. **Start with 01** - Learn async basics and syntax
2. **Progress to 02-04** - Understand task spawning and async I/O
3. **Move to 05-06** - Practice data processing
4. **Finish with 07-08** - Master production patterns with Redis

## 💡 Tips for Beginners

- Read the comments in each file carefully
- Run each example and observe the output
- Try modifying sleep durations and message content
- Experiment with adding more tasks or channels
- Check the error messages - Rust's compiler is very helpful!

## 🤝 Contributing

This is a learning project. Feel free to:
- Add more examples
- Improve documentation
- Fix bugs or typos
- Suggest better patterns

## 📝 License

This project is for educational purposes.

## 👤 Author

Created as a learning resource for Rust beginners focusing on async programming.

---

**Happy Learning! 🦀**

