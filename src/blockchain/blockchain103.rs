// represent a block from a blockchain, using Rust structs
use sha2::{Sha256, Digest};
use std::fmt::Write;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub transaction_hash: String,
    pub sender_wallet_id: String,
    pub receiver_wallet_id: String,
    pub amount: f64,
    pub note: Option<String>,
    pub signature: String,
    pub block_index: Option<i64>,
    pub transaction_type: String,
    pub timestamp: i64,
    pub created_at: DateTime<Utc>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub txn_data: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub merkle_root: Option<String>,
}

impl Block {
    // Create a new block
    pub fn new(id: u64, previous_hash: String, txn_data: String) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        let mut block = Block {
            id,
            hash: String::new(),
            previous_hash,
            timestamp,
            txn_data,
            nonce: 0,
            transactions: Vec::new(),
            merkle_root: None,
        };
        block.merkle_root = Some(block.calculate_merkle_root());
        block.hash = block.calculate_hash();
        block
    }

    // Create a new block with transactions
    pub fn new_with_transactions(id: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        let mut block = Block {
            id,
            hash: String::new(),
            previous_hash,
            timestamp,
            txn_data: format!("{} transactions", transactions.len()),
            nonce: 0,
            transactions,
            merkle_root: None,
        };
        block.merkle_root = Some(block.calculate_merkle_root());
        block.hash = block.calculate_hash();
        block
    }

    // Calculate merkle root from transactions
    pub fn calculate_merkle_root(&self) -> String {
        if self.transactions.is_empty() {
            return String::from("0000000000000000000000000000000000000000000000000000000000000000");
        }

        let mut hashes: Vec<String> = self.transactions
            .iter()
            .map(|tx| tx.transaction_hash.clone())
            .collect();

        // Build merkle tree
        while hashes.len() > 1 {
            let mut new_level = Vec::new();
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    // Duplicate last hash if odd number
                    format!("{}{}", chunk[0], chunk[0])
                };
                let mut hasher = Sha256::new();
                hasher.update(combined.as_bytes());
                let result = hasher.finalize();
                let mut hash_string = String::new();
                for byte in result.iter() {
                    write!(&mut hash_string, "{:02x}", byte).unwrap();
                }
                new_level.push(hash_string);
            }
            hashes = new_level;
        }

        hashes[0].clone()
    }

    // Calculate hash of the block
    pub fn calculate_hash(&self) -> String {
        let default_merkle = String::from("0");
        let merkle = self.merkle_root.as_ref().unwrap_or(&default_merkle);
        let data = format!(
            "{}{}{}{}{}{}",
            self.id, self.previous_hash, self.timestamp, self.txn_data, self.nonce, merkle
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        // Convert hash to hex string
        let mut hash_string = String::new();
        for byte in result.iter() {
            write!(&mut hash_string, "{:02x}", byte).unwrap();
        }
        hash_string
    }

    // Mine block with proof-of-work (difficulty = number of leading zeros)
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        println!("Mining block {}...", self.id);

        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("Block mined! Hash: {}", self.hash);
        println!("Nonce: {}\n", self.nonce);
    }
}

// blockchain can be represented
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    // Create a new blockchain with genesis block
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            difficulty,
        };

        // Create genesis block
        let mut genesis = Block::new(0, String::from("0"), String::from("Genesis Block"));
        genesis.mine_block(difficulty);
        blockchain.blocks.push(genesis);

        blockchain
    }

    // Get the latest block
    pub fn get_latest_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    // Add a new block with transactions to the blockchain
    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.get_latest_block().hash.clone();
        let id = self.blocks.len() as u64;

        let mut new_block = Block::new_with_transactions(id, previous_hash, transactions);
        new_block.mine_block(self.difficulty);

        self.blocks.push(new_block);
    }

    // Validate a single block
    pub fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        // Check if previous hash matches
        if block.previous_hash != previous_block.hash {
            println!("❌ Invalid previous hash for block {}", block.id);
            return false;
        }

        // Check if hash is correct
        if block.hash != block.calculate_hash() {
            println!("❌ Invalid hash for block {}", block.id);
            return false;
        }

        // Check proof-of-work
        let target = "0".repeat(self.difficulty);
        if &block.hash[..self.difficulty] != target {
            println!("❌ Block {} doesn't meet difficulty requirement", block.id);
            return false;
        }

        true
    }

    // Validate the entire blockchain
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];

            if !self.is_block_valid(current_block, previous_block) {
                return false;
            }
        }

        println!("✅ Blockchain is valid!");
        true
    }

    // Display blockchain
    pub fn display(&self) {
        println!("\n{}", "=".repeat(80));
        println!("BLOCKCHAIN (Difficulty: {})", self.difficulty);
        println!("{}\n", "=".repeat(80));

        for block in &self.blocks {
            println!("--- Block {} ---", block.id);
            println!("Hash:          {}", block.hash);
            println!("Previous Hash: {}", block.previous_hash);
            println!("Timestamp:     {}", block.timestamp);
            println!("Data:          {}", block.txn_data);
            println!("Nonce:         {}", block.nonce);
            println!("Merkle Root:   {}", block.merkle_root.as_ref().unwrap_or(&String::from("N/A")));
            println!("Transactions:  {} transaction(s)", block.transactions.len());
            println!();
        }
    }
}


// Helper function to create a transaction
fn create_transaction(
    sender: &str,
    receiver: &str,
    amount: f64,
    transaction_type: &str,
) -> Transaction {
    let id = Uuid::new_v4();
    let timestamp = Utc::now().timestamp();
    let created_at = Utc::now();

    // Create a simple transaction hash
    let data = format!("{}{}{}{}", sender, receiver, amount, timestamp);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    let mut transaction_hash = String::new();
    for byte in result.iter() {
        write!(&mut transaction_hash, "{:02x}", byte).unwrap();
    }

    Transaction {
        id,
        transaction_hash,
        sender_wallet_id: sender.to_string(),
        receiver_wallet_id: receiver.to_string(),
        amount,
        note: Some(format!("Payment of {} BTC", amount)),
        signature: format!("sig_{}", id),
        block_index: None,
        transaction_type: transaction_type.to_string(),
        timestamp,
        created_at,
    }
}

fn main() {
    println!("🔗 Custom Blockchain Implementation\n");

    // Create a new blockchain with difficulty 3 (3 leading zeros for faster demo)
    let mut blockchain = Blockchain::new(3);

    println!("\n--- Adding Blocks with Transactions ---\n");

    // Block 1: Alice pays Bob
    let block1_transactions = vec![
        create_transaction("wallet_alice", "wallet_bob", 10.0, "transfer"),
    ];
    blockchain.add_block(block1_transactions);

    // Block 2: Bob pays Charlie
    let block2_transactions = vec![
        create_transaction("wallet_bob", "wallet_charlie", 5.0, "transfer"),
    ];
    blockchain.add_block(block2_transactions);

    // Block 3: Multiple transactions
    let block3_transactions = vec![
        create_transaction("wallet_alice", "wallet_bob", 15.5, "transfer"),
        create_transaction("wallet_bob", "wallet_charlie", 7.25, "transfer"),
        create_transaction("wallet_charlie", "wallet_david", 3.0, "transfer"),
    ];
    blockchain.add_block(block3_transactions);

    // Block 4: More transactions
    let block4_transactions = vec![
        create_transaction("wallet_david", "wallet_eve", 12.0, "transfer"),
        create_transaction("wallet_eve", "wallet_frank", 8.5, "transfer"),
    ];
    blockchain.add_block(block4_transactions);

    // Display the blockchain
    blockchain.display();

    // Validate the blockchain
    println!("\n--- Validating Blockchain ---\n");
    blockchain.is_chain_valid();

    // Demonstrate merkle root integrity
    println!("\n--- Demonstrating Merkle Root Integrity ---\n");
    let block_3 = &blockchain.blocks[3];
    println!("Block {} has {} transactions", block_3.id, block_3.transactions.len());
    println!("Merkle Root: {}", block_3.merkle_root.as_ref().unwrap());
    println!("This merkle root represents all transactions in the block cryptographically.");

    // Demonstrate tampering detection
    println!("\n--- Testing Tampering Detection ---\n");
    println!("Attempting to tamper with block 1's transaction amount...");
    if let Some(tx) = blockchain.blocks[1].transactions.first_mut() {
        tx.amount = 100.0; // Tamper with the amount (from 10.0 to 100.0)
    }

    println!("Validating blockchain after tampering...\n");
    if !blockchain.is_chain_valid() {
        println!("❌ Tampering detected! Blockchain is invalid.");
    }

    println!("\n✨ Blockchain demonstration complete!");
}