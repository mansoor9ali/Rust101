 

use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Mini-Redis Client Examples ===\n");

    // Open a connection to the mini-redis address.
    println!("Connecting to Redis server at 127.0.0.1:6379...");
    let mut client = client::connect("127.0.0.1:6379").await?;
    println!("✓ Connected!\n");

    // Example 1: Set and Get a simple message
    println!("Example 1: Basic SET and GET");
    println!("Setting key 'greeting' = 'Hello from Rust!'");
    client.set("greeting", "Hello from Rust!".into()).await?;

    let result = client.get("greeting").await?;
    println!("Got value: {:?}\n", result);

    // Example 2: Store user information
    println!("Example 2: Storing user information");
    client.set("user:name", "Mansoor Ali".into()).await?;
    client.set("user:email", "mansoor.ali@live.com".into()).await?;
    client.set("user:role", "Rust Developer".into()).await?;

    let name = client.get("user:name").await?;
    let email = client.get("user:email").await?;
    let role = client.get("user:role").await?;

    println!("User Profile:");
    println!("  Name: {:?}", name);
    println!("  Email: {:?}", email);
    println!("  Role: {:?}\n", role);

    // Example 3: Store and retrieve messages
    println!("Example 3: Message queue simulation");
    let messages = vec![
        "First message",
        "Second message",
        "Third message",
    ];

    for (i, msg) in messages.iter().enumerate() {
        let key = format!("message:{}", i + 1);
        println!("Storing: {} = '{}'", key, msg);
        client.set(&key, (*msg).into()).await?;
    }

    println!("\nRetrieving messages:");
    for i in 1..=3 {
        let key = format!("message:{}", i);
        if let Some(msg) = client.get(&key).await? {
            println!("Retrieved {}: {:?}", key, msg);
        }
    }

    // Example 4: Update a value
    println!("\nExample 4: Updating a value");
    client.set("counter", "100".into()).await?;
    println!("Initial counter value: {:?}", client.get("counter").await?);

    client.set("counter", "200".into()).await?;
    println!("Updated counter value: {:?}", client.get("counter").await?);

    // Example 5: Handle non-existent keys
    println!("\nExample 5: Checking non-existent key");
    let non_existent = client.get("does_not_exist").await?;
    match non_existent {
        Some(value) => println!("Found value: {:?}", value),
        None => println!("Key 'does_not_exist' not found (returns None)"),
    }

    println!("\n✅ All operations completed successfully!");
    Ok(())
}