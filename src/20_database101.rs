use deadpool_postgres::{Pool, Client};
use rust101::models::User;
// Database connection and operations module

// The DbPool type alias is kept for convenience
pub type DbPool = Pool;

// The central struct to hold our connection pool
#[derive(Debug)]
pub struct Database {
    pool: DbPool,
}


impl Database {
    pub async fn new() -> Result<Database, Box<dyn std::error::Error>> {
        let database_url = "postgresql://user-name:strong-password@127.0.0.1:5432/blockchain101".to_string();

        let mut cfg = deadpool_postgres::Config::new();
        cfg.url = Some(database_url);

        // Limit pool size for Supabase free tier
        cfg.pool = Some(deadpool_postgres::PoolConfig::new(10));

        cfg.manager = Some(deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        });

        let pool = cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), tokio_postgres::NoTls)?;

        println!("✅ Database connection pool created");

        Ok(Database { pool })
    }

    // Helper method to get a connection from the pool
    pub async fn get_client(&self) -> Result<Client, Box<dyn std::error::Error>> {
        self.pool.get().await.map_err(|e| {
            println!("Failed to get client from pool: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })
    }

    // User queries

    pub async fn create_user(
        &self,
        email: &str,
        full_name: &str,
        cnic: &str,
        wallet_id: &str,
        public_key: &str,
        encrypted_private_key: &str,
    ) -> Result<User, Box<dyn std::error::Error>> {
        // 1. Acquire client connection
        let client = self.get_client().await?;
        // 2. Execute insert query using the client
        let row = client
            .query_one(
                "INSERT INTO users (email, full_name, cnic, wallet_id, public_key, encrypted_private_key)
                 VALUES ($1, $2, $3, $4, $5, $6)
                 RETURNING id, email, full_name, cnic, wallet_id, public_key, encrypted_private_key, is_verified, created_at, updated_at",
                &[&email, &full_name, &cnic, &wallet_id, &public_key, &encrypted_private_key],
            )
            .await?;
        // 3. Mapresult
        Ok(User {
            id: row.get(0),
            email: row.get(1),
            full_name: row.get(2),
            cnic: row.get(3),
            wallet_id: row.get(4),
            public_key: row.get(5),
            encrypted_private_key: row.get(6),
            is_verified: row.get(7),
            created_at: row.get(8),
            updated_at: row.get(9),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example usage
    // Create a new database instance
    let db = Database::new().await?;
    println!("Database initialized successfully!");


    // Create 10 users with sample data
    for i in 1..=10 {
        let email = format!("user{}@example.com", i);
        let full_name = format!("User {}", i);
        let cnic = format!("1234567890{:02}", i);
        let wallet_id = format!("wallet_{}", i);
        let public_key = format!("pubkey_{}", i);
        let encrypted_private_key = format!("encrypted_privkey_{}", i);

        match db.create_user(
            &email,
            &full_name,
            &cnic,
            &wallet_id,
            &public_key,
            &encrypted_private_key
        ).await {
            Ok(user) => {
                println!("✅ User {} created: {} ({})", i, user.email, user.wallet_id);
            }
            Err(e) => {
                println!("❌ Failed to create user {}: {}", i, e);
            }
        }
    }




    Ok(())
}