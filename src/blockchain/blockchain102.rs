// represent a block from a blockchain, using Rust structs
use sha2::{Sha256, Digest};
use std::fmt::Write;

pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub txn_data: String,
    pub nonce: u64,
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
        };
        block.hash = block.calculate_hash();
        block
    }

    // Calculate hash of the block
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.id, self.previous_hash, self.timestamp, self.txn_data, self.nonce
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

    // Add a new block to the blockchain
    pub fn add_block(&mut self, txn_data: String) {
        let previous_hash = self.get_latest_block().hash.clone();
        let id = self.blocks.len() as u64;

        let mut new_block = Block::new(id, previous_hash, txn_data);
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
            println!();
        }
    }
}


fn main() {
    println!("🔗 Custom Blockchain Implementation\n");

    // Create a new blockchain with difficulty 4 (4 leading zeros)
    let mut blockchain = Blockchain::new(4);

    println!("\n--- Adding Blocks to Blockchain ---\n");

    // Add transaction blocks
    blockchain.add_block(String::from("Alice pays Bob 10 BTC"));
    blockchain.add_block(String::from("Bob pays Charlie 5 BTC"));
    blockchain.add_block(String::from("Charlie pays David 3 BTC"));
    blockchain.add_block(String::from("David pays Eve 7 BTC"));

    // Display the blockchain
    blockchain.display();

    // Validate the blockchain
    println!("\n--- Validating Blockchain ---\n");
    blockchain.is_chain_valid();

    // Demonstrate tampering detection
    println!("\n--- Testing Tampering Detection ---\n");
    println!("Attempting to tamper with block 2...");
    blockchain.blocks[2].txn_data = String::from("Bob pays Charlie 100 BTC (TAMPERED)");

    println!("Validating blockchain after tampering...\n");
    if !blockchain.is_chain_valid() {
        println!("❌ Tampering detected! Blockchain is invalid.");
    }

    println!("\n✨ Blockchain demonstration complete!");
}