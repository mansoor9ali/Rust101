use dotenv::dotenv;
fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    // Access an environment variable
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);
}