use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
   let handle= tokio::spawn(async move {
        hello().await;

    });
    world().await;
    handle.await.unwrap();
}

async fn hello() {
    println!("Hello, async fn");
    sleep(Duration::from_secs(2)).await;
    println!("Hello fn Sleeping 2 seconds");
    println!("hello async fn complete!");

}

async fn world() {
    println!("world, async fn!");
    sleep(Duration::from_secs(1)).await;
    println!("world fn Sleeping 1 seconds");
    println!("world, async fn complete!");
}