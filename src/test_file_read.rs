use tokio::io::AsyncReadExt;
#[tokio::main]
async fn main() {
    let mut file = tokio::fs::File::open("./data/hello.txt").await.unwrap();
    let mut buf = vec![0; 1024];
    file.read(&mut buf).await.unwrap();
    let content = String::from_utf8_lossy(&buf);
    println!("File content: {}", content);
}