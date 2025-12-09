//! Simple Sled Database Example
//!
//! This program demonstrates the basic usage of sled, an embedded database for Rust.
//! Sled is a modern embedded database with:
//! - ACID transactions
//! - Zero-copy reads
//! - Log-structured storage
//! - Compression
//! - Thread-safe operations

use serde::{Deserialize, Serialize};
use sled::{Db, IVec};

/// A simple User struct to demonstrate storing complex data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}

impl User {
    fn new(id: u32, name: &str, email: &str, age: u32) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            age,
        }
    }

    /// Convert User to bytes for storage
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    /// Convert bytes back to User
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let user = serde_json::from_slice(bytes)?;
        Ok(user)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Sled Database Tutorial ===\n");

    // 1. Open/Create a database
    println!("1. Opening database...");
    let db: Db = sled::open("my_sled_db")?;
    println!("   ✓ Database opened successfully!\n");

    // 2. Basic Key-Value Operations
    println!("2. Basic Key-Value Operations:");

    // Insert a simple key-value pair
    db.insert(b"greeting", b"Hello, Sled!")?;
    println!("   ✓ Inserted: greeting -> Hello, Sled!");

    // Retrieve the value
    if let Some(value) = db.get(b"greeting")? {
        let greeting = std::str::from_utf8(&value)?;
        println!("   ✓ Retrieved: {}", greeting);
    }

    // Insert more data
    db.insert(b"language", b"Rust")?;
    db.insert(b"year", b"2025")?;
    println!("   ✓ Inserted multiple key-value pairs\n");

    // 3. Working with Numbers
    println!("3. Working with Numbers:");
    let counter: u64 = 42;
    db.insert(b"counter", &counter.to_be_bytes())?;

    if let Some(value) = db.get(b"counter")? {
        let bytes: [u8; 8] = value.as_ref().try_into()?;
        let retrieved_counter = u64::from_be_bytes(bytes);
        println!("   ✓ Counter stored and retrieved: {}", retrieved_counter);
    }
    println!();

    // 4. Storing Complex Data (JSON serialization)
    println!("4. Storing Complex Data:");
    let user1 = User::new(1, "Alice Johnson", "alice@example.com", 28);
    let user2 = User::new(2, "Bob Smith", "bob@example.com", 35);
    let user3 = User::new(3, "Charlie Brown", "charlie@example.com", 42);

    db.insert(format!("user:{}", user1.id).as_bytes(), user1.to_bytes())?;
    db.insert(format!("user:{}", user2.id).as_bytes(), user2.to_bytes())?;
    db.insert(format!("user:{}", user3.id).as_bytes(), user3.to_bytes())?;
    println!("   ✓ Stored 3 users");

    // Retrieve a user
    if let Some(value) = db.get(b"user:1")? {
        let user = User::from_bytes(&value)?;
        println!("   ✓ Retrieved user: {:?}", user);
    }
    println!();

    // 5. Iteration - Scan all keys
    println!("5. Iterating over all keys:");
    for item in db.iter() {
        let (key, _value) = item?;
        let key_str = String::from_utf8_lossy(&key);
        println!("   - Key: {}", key_str);
    }
    println!();

    // 6. Prefix Scan - Get all users
    println!("6. Prefix Scan (all users):");
    for item in db.scan_prefix(b"user:") {
        let (key, value) = item?;
        let key_str = String::from_utf8_lossy(&key);
        if let Ok(user) = User::from_bytes(&value) {
            println!("   - {}: {} (age: {})", key_str, user.name, user.age);
        }
    }
    println!();

    // 7. Update Operation
    println!("7. Update Operation:");
    let mut updated_user = user1.clone();
    updated_user.age = 29;
    db.insert(b"user:1", updated_user.to_bytes())?;

    if let Some(value) = db.get(b"user:1")? {
        let user = User::from_bytes(&value)?;
        println!("   ✓ Updated user age: {} -> {}", user1.age, user.age);
    }
    println!();

    // 8. Delete Operation
    println!("8. Delete Operation:");
    db.remove(b"user:3")?;
    println!("   ✓ Deleted user:3 (Charlie Brown)");

    let count_before = db.scan_prefix(b"user:").count();
    println!("   ✓ Remaining users: {}", count_before);
    println!();

    // 9. Atomic Operations - Compare and Swap
    println!("9. Atomic Compare-and-Swap:");
    db.insert(b"atomic_counter", &0u64.to_be_bytes())?;

    // Increment counter atomically
    for _i in 1..=5 {
        db.update_and_fetch(b"atomic_counter", |old_value| {
            let old = if let Some(v) = old_value {
                let bytes: [u8; 8] = v.try_into().ok()?;
                u64::from_be_bytes(bytes)
            } else {
                0
            };
            Some(IVec::from(&(old + 1).to_be_bytes()))
        })?;
    }

    if let Some(value) = db.get(b"atomic_counter")? {
        let bytes: [u8; 8] = value.as_ref().try_into()?;
        let final_count = u64::from_be_bytes(bytes);
        println!("   ✓ Atomic counter after 5 increments: {}", final_count);
    }
    println!();

    // 10. Database Statistics
    println!("10. Database Statistics:");
    let size = db.size_on_disk()?;
    println!("   - Database size on disk: {} bytes", size);
    println!("   - Total keys: {}", db.len());
    println!();

    // 11. Flush to ensure all data is written to disk
    println!("11. Flushing to disk...");
    db.flush()?;
    println!("   ✓ All data flushed to disk\n");

    // 12. Clear specific prefix
    println!("12. Batch Delete by Prefix:");
    let keys_to_delete: Vec<_> = db.scan_prefix(b"user:")
        .filter_map(|item| item.ok())
        .map(|(key, _)| key)
        .collect();

    for key in keys_to_delete {
        db.remove(key)?;
    }
    println!("   ✓ Deleted all users");
    println!("   ✓ Remaining user keys: {}", db.scan_prefix(b"user:").count());
    println!();

    println!("=== Tutorial Complete! ===");
    println!("\nKey Takeaways:");
    println!("  • Sled is simple: open() -> insert()/get() -> flush()");
    println!("  • It's thread-safe: share Db across threads");
    println!("  • Zero-copy reads: very fast retrieval");
    println!("  • Use serde for complex data structures");
    println!("  • Supports atomic operations and transactions");
    println!("  • Perfect for embedded use cases!");

    Ok(())
}

