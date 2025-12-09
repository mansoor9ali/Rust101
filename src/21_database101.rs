use deadpool_postgres::{Pool, Client};
use tokio_postgres::Error as PgError;
use tokio_postgres::NoTls;
use std::env;
use crate::models::*;
use crate::models::Transaction as TxModel;
// The DbPool type alias is kept for convenience
pub type DbPool = Pool;

// The central struct to hold our connection pool
pub struct Database {
    pool: DbPool,
}


impl Database {
    pub async fn new() -> Result<Database, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL")?;

        let mut cfg = deadpool_postgres::Config::new();
        cfg.url = Some(database_url);

        // Limit pool size for Supabase free tier
        cfg.pool = Some(deadpool_postgres::PoolConfig::new(10));

        cfg.manager = Some(deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        });

        let pool = cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), tokio_postgres::NoTls)?;

        log::info!("✅ Database connection pool created");

        Ok(Database { pool })
    }

    // Helper method to get a connection from the pool
    pub async fn get_client(&self) -> Result<Client, PgError> {
        self.pool.get().await.map_err(|e| {
            log::error!("Failed to get client from pool: {}", e);
            PgError::from(e)
        })
    }

    // User queries

    pub async fn create_user(
        email: &str,
        full_name: &str,
        cnic: &str,
        wallet_id: &str,
        public_key: &str,
        encrypted_private_key: &str,
    ) -> Result<User, tokio_postgres::Error> {
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
    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, PgError> {
        // 1. Acquire client connection
        let client = self.get_client().await?;

        // 2. Execute query using the client
        let result = client
            .query_opt(
                "SELECT id, email, full_name, cnic, wallet_id, public_key, encrypted_private_key, is_verified, created_at, updated_at
                 FROM users WHERE email = $1",
                &[&email],
            )
            .await?;

        // 3. Map result
        Ok(result.map(|row| User {
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
        }))
    }

    // Wallet queries
    pub async fn update_wallet_balance(&self, wallet_id: &str, new_balance: f64) -> Result<(), PgError> {
        let client = self.get_client().await?;
        client
            .execute(
                "UPDATE wallets SET balance = $1::float8, updated_at = $2 WHERE wallet_id = $3",
                &[&new_balance, &chrono::Utc::now(), &wallet_id],
            )
            .await?;
        Ok(())
    }
}