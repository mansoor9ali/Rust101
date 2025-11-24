// ==================== CHAPTER 1: Async Basics with Tokio ====================
// Tokio is an async runtime for Rust. It allows you to write asynchronous code
// that can handle many tasks concurrently without blocking.


//
// // ==================== KEY CONCEPTS ====================
// //
// // 1. ASYNC/AWAIT:
// //    - async fn creates an asynchronous function
// //    - .await waits for an async operation to complete
// //    - Allows non-blocking execution
// //
// // 2. TOKIO::SPAWN:
// //    - Creates a new task that runs concurrently
// //    - Returns a JoinHandle you can await
// //    - Tasks run on the Tokio runtime
// //
// // 3. TOKIO::JOIN!:
// //    - Waits for multiple async operations concurrently
// //    - More efficient than awaiting sequentially
// //    - Returns a tuple of results
// //
// // 4. CHANNELS:
// //    - mpsc = Multi-Producer, Single-Consumer
// //    - Allows communication between tasks
// //    - Sender (tx) and Receiver (rx)
// //
mod rs_async_basics;

#[tokio::main]
async fn main() {
    println!("=== CHAPTER 1: Async Basics ===\n");
    chapter1_async_basics().await;

    println!("\n=== CHAPTER 2: Concurrent Tasks ===\n");
    chapter2_concurrent_tasks().await;

    println!("\n=== CHAPTER 3: Async Communication ===\n");
    chapter3_async_communication().await;
}

// Chapter 1: Understanding async/await and basic delays
async fn chapter1_async_basics() {
    println!("1. Simple async function");
    greet_async("Rust Developer").await;

    println!("\n2. Async delays");
    println!("Starting countdown...");
    for i in (1..=3).rev() {
        println!("{}", i);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    println!("Go!");



    println!("\n3. Async function with return value");
    let result = fetch_data().await;
    println!("Fetched: {}", result);
}

async fn greet_async(name: &str) {
    // Simulate some async work
    tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    println!("Hello, {}!", name);
}

async fn fetch_data() -> String {
    // Simulate fetching data from a database or API
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    "Important data from async operation".to_string()
}

// ==================== CHAPTER 2: Concurrent Tasks ====================
// Tokio allows multiple tasks to run concurrently using tokio::spawn

async fn chapter2_concurrent_tasks() {
    println!("1. Running tasks concurrently with tokio::spawn");

    // Spawn three tasks that run concurrently
    let task1 = tokio::spawn(async {
        println!("Task 1: Started");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("Task 1: Completed");
        "Result from Task 1"
    });

    let task2 = tokio::spawn(async {
        println!("Task 2: Started");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("Task 2: Completed");
        "Result from Task 2"
    });

    let task3 = tokio::spawn(async {
        println!("Task 3: Started");
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        println!("Task 3: Completed");
        "Result from Task 3"
    });

    // Wait for all tasks to complete
    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();
    let result3 = task3.await.unwrap();

    println!("\nAll tasks completed!");
    println!("{}, {}, {}", result1, result2, result3);

    println!("\n2. Using tokio::join! to run tasks concurrently");
    let (r1, r2, r3) = tokio::join!(
        download_file("file1.txt"),
        download_file("file2.txt"),
        download_file("file3.txt")
    );
    println!("Downloaded: {}, {}, {}", r1, r2, r3);
}

async fn download_file(filename: &str) -> String {
    println!("Downloading {}...", filename);
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("{} downloaded!", filename);
    format!("Content of {}", filename)
}

// ==================== CHAPTER 3: Async Communication ====================
// Tokio provides channels for communication between async tasks

async fn chapter3_async_communication() {
    println!("1. Using channels for task communication");

    // Create a channel (sender and receiver)
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);

    // Spawn a producer task
    tokio::spawn(async move {
        let messages = vec!["Hello", "from", "async", "channel"];
        for msg in messages {
            println!("Sending: {}", msg);
            tx.send(msg.to_string()).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });

    // Receive messages in the main task
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }

    println!("\n2. Multiple producers, single consumer");
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);

    // Spawn multiple producer tasks
    for i in 1..=3 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            for j in 1..=2 {
                let msg = format!("Message {} from Producer {}", j, i);
                tx_clone.send(msg).await.unwrap();
                tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
            }
        });
    }

    // Drop the original sender so the channel closes when all clones are dropped
    drop(tx);

    // Receive all messages
    while let Some(message) = rx.recv().await {
        println!("Consumer received: {}", message);
    }

    println!("\n✅ All chapters completed!");
}