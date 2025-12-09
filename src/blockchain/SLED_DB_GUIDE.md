# Sled Database Example

This is a comprehensive tutorial demonstrating how to use **sled**, an embedded database for Rust.

## What is Sled?

Sled is a modern embedded database written in Rust that provides:
- **ACID transactions**: Ensures data consistency
- **Zero-copy reads**: Very fast data retrieval
- **Log-structured storage**: Efficient write performance
- **Compression**: Reduces disk space usage
- **Thread-safe operations**: Safe to share across threads
- **No external dependencies**: Embedded directly in your application

## Running the Example

```bash
cargo run --bin sled_db_example
```

## What This Example Covers

### 1. **Opening/Creating a Database**
   - Simple database initialization with `sled::open()`

### 2. **Basic Key-Value Operations**
   - Inserting simple string data
   - Retrieving values
   - Working with byte arrays

### 3. **Working with Numbers**
   - Storing and retrieving numeric data
   - Using byte conversion for primitive types

### 4. **Storing Complex Data**
   - Serializing structs to JSON
   - Storing complex objects
   - Deserializing data back to structs

### 5. **Iteration**
   - Scanning all keys in the database
   - Iterating over database contents

### 6. **Prefix Scanning**
   - Efficiently querying keys with a common prefix
   - Useful for namespacing data (e.g., `user:1`, `user:2`)

### 7. **Update Operations**
   - Modifying existing data
   - Overwriting values

### 8. **Delete Operations**
   - Removing individual keys
   - Counting remaining entries

### 9. **Atomic Operations**
   - Compare-and-swap operations
   - Thread-safe counter increments
   - Using `update_and_fetch` for atomic updates

### 10. **Database Statistics**
   - Getting disk usage
   - Counting total keys

### 11. **Flushing to Disk**
   - Ensuring data persistence
   - Manual flush operations

### 12. **Batch Operations**
   - Deleting multiple keys by prefix
   - Bulk operations

## Key Concepts

### User Struct Example
The example includes a `User` struct that demonstrates:
- Serialization with serde
- Converting structs to/from bytes
- Storing complex data structures

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}
```

### Byte Conversion
Sled stores everything as bytes, so you need to:
- Convert data to bytes before storing
- Convert bytes back to original type when retrieving
- Use `serde_json` for complex types
- Use `to_be_bytes()`/`from_be_bytes()` for primitives

## Use Cases for Sled

- **Embedded applications**: No separate database server needed
- **Configuration storage**: Store app settings
- **Caching layer**: Fast local data cache
- **State management**: Persist application state
- **Blockchain nodes**: Store blockchain data locally
- **Desktop applications**: Local data storage
- **IoT devices**: Lightweight database for embedded systems

## Performance Characteristics

- **Fast reads**: Zero-copy architecture
- **Efficient writes**: Log-structured merge trees
- **Low memory footprint**: Suitable for embedded systems
- **Crash-safe**: ACID guarantees data integrity

## File Location

- Source: `src/blockchain/test.rs`
- Binary name: `sled_db_example`
- Database directory: `my_sled_db/` (created automatically)

## Further Reading

- [Sled Documentation](https://docs.rs/sled/)
- [Sled GitHub Repository](https://github.com/spacejam/sled)
- [Sled Book](https://sled.rs/)

## Tips

1. **Always flush**: Call `db.flush()` when you want to ensure data is persisted
2. **Use prefixes**: Organize related data with key prefixes (e.g., `user:`, `config:`)
3. **Serialize complex types**: Use serde for structs and enums
4. **Thread-safe**: You can safely clone and share `Db` across threads
5. **Atomic operations**: Use `update_and_fetch` for concurrent modifications

