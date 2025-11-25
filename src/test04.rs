use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender1, mut receiver ) = mpsc::channel(10);
    let sender2 = sender1.clone();
    tokio::spawn(async move {
        for i in 0..5 {
            sender1.send(format!("Message from sender1: {}", i)).await.unwrap();
        }
    });

    tokio::spawn(async move {
        for i in 0..5 {
            sender2.send(format!("Message from sender2: {}", i)).await.unwrap();
        }
    });

    while let Some(message) = receiver.recv().await {
        println!("Received: {}", message);
    }
}