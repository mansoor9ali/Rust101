// 🔗 Blockchain 104: UTXO Model, Wallets & Digital Signatures
// This implementation adds Bitcoin-like UTXO (Unspent Transaction Output) model,
// wallet system with public/private keys, and transaction signing/verification.

use sha2::{Sha256, Digest};
use std::fmt::Write;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ================================================================================================
// CORE DATA STRUCTURES
// ================================================================================================

/// Transaction Input - references a previous transaction output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXInput {
    pub txid: String,              // Transaction ID being spent
    pub vout: usize,               // Output index in that transaction
    pub signature: String,         // Signature proving ownership
    pub pub_key: String,           // Public key of sender
}

impl TXInput {
    pub fn new(txid: String, vout: usize, signature: String, pub_key: String) -> Self {
        TXInput {
            txid,
            vout,
            signature,
            pub_key,
        }
    }

    /// Check if this input can be unlocked by a public key
    pub fn can_unlock_output_with(&self, pub_key_hash: &str) -> bool {
        let input_pub_key_hash = hash_pub_key(&self.pub_key);
        input_pub_key_hash == pub_key_hash
    }
}

/// Transaction Output - represents coins that can be spent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TXOutput {
    pub value: i32,                // Amount of coins
    pub pub_key_hash: String,      // Hash of public key (address)
}

impl TXOutput {
    pub fn new(value: i32, address: &str) -> Self {
        let mut output = TXOutput {
            value,
            pub_key_hash: String::new(),
        };
        output.lock(address);
        output
    }

    /// Lock output to an address
    fn lock(&mut self, address: &str) {
        // In real implementation, this would decode base58 address
        // For educational purposes, we'll use the address directly
        self.pub_key_hash = address.to_string();
    }

    /// Check if output can be unlocked by a public key
    pub fn can_be_unlocked_with(&self, pub_key_hash: &str) -> bool {
        self.pub_key_hash == pub_key_hash
    }
}

/// Transaction - with UTXO model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,                    // Transaction hash
    pub vin: Vec<TXInput>,             // Inputs (coins being spent)
    pub vout: Vec<TXOutput>,           // Outputs (new coins)
    pub timestamp: i64,
}

impl Transaction {
    /// Create a coinbase transaction (mining reward)
    pub fn new_coinbase(to: &str, data: Option<String>) -> Self {
        let timestamp = Utc::now().timestamp();
        let txout = TXOutput::new(50, to); // 50 coins reward

        // Coinbase has no real input
        let txin = TXInput {
            txid: String::new(),
            vout: 0,
            signature: data.unwrap_or_else(|| format!("Reward to {}", to)),
            pub_key: String::new(),
        };

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![txin],
            vout: vec![txout],
            timestamp,
        };
        tx.id = tx.calculate_hash();
        tx
    }

    /// Create a regular UTXO transaction
    pub fn new_utxo_transaction(
        from_wallet: &Wallet,
        to: &str,
        amount: i32,
        utxo_set: &HashMap<String, Vec<TXOutput>>,
    ) -> Result<Self, String> {
        let from_pub_key_hash = hash_pub_key(&from_wallet.public_key);

        // Find spendable outputs
        let (accumulated, valid_outputs) =
            find_spendable_outputs(&from_pub_key_hash, amount, utxo_set);

        if accumulated < amount {
            return Err(format!(
                "Not enough funds! Need {}, have {}",
                amount, accumulated
            ));
        }

        // Build inputs
        let mut inputs = vec![];
        for (txid, outputs) in valid_outputs {
            for out_idx in outputs {
                let signature = from_wallet.sign(&txid);
                let txin = TXInput::new(
                    txid.clone(),
                    out_idx,
                    signature,
                    from_wallet.public_key.clone(),
                );
                inputs.push(txin);
            }
        }

        // Build outputs
        let mut outputs = vec![TXOutput::new(amount, to)];

        // Add change output if necessary
        if accumulated > amount {
            let change = accumulated - amount;
            outputs.push(TXOutput::new(change, &from_wallet.get_address()));
        }

        let mut tx = Transaction {
            id: String::new(),
            vin: inputs,
            vout: outputs,
            timestamp: Utc::now().timestamp(),
        };
        tx.id = tx.calculate_hash();
        Ok(tx)
    }

    /// Check if transaction is coinbase
    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].txid.is_empty()
    }

    /// Calculate transaction hash
    fn calculate_hash(&self) -> String {
        let data = format!(
            "{:?}{:?}{}",
            self.vin, self.vout, self.timestamp
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        let mut hash_string = String::new();
        for byte in result.iter() {
            write!(&mut hash_string, "{:02x}", byte).unwrap();
        }
        hash_string
    }

    /// Verify transaction signatures
    pub fn verify(&self, utxo_set: &HashMap<String, Vec<TXOutput>>) -> bool {
        if self.is_coinbase() {
            return true;
        }

        // Verify each input
        for input in &self.vin {
            // Find the output being spent
            if let Some(outputs) = utxo_set.get(&input.txid) {
                if let Some(output) = outputs.get(input.vout) {
                    let pub_key_hash = hash_pub_key(&input.pub_key);
                    if !output.can_be_unlocked_with(&pub_key_hash) {
                        println!("❌ Invalid signature for input");
                        return false;
                    }
                } else {
                    println!("❌ Output index {} not found", input.vout);
                    return false;
                }
            } else {
                println!("❌ Transaction {} not found in UTXO set", input.txid);
                return false;
            }
        }
        true
    }
}

/// Block structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub merkle_root: String,
}

impl Block {
    /// Create new block with transactions
    pub fn new(id: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            id,
            hash: String::new(),
            previous_hash,
            timestamp,
            nonce: 0,
            transactions,
            merkle_root: String::new(),
        };
        block.merkle_root = block.calculate_merkle_root();
        block.hash = block.calculate_hash();
        block
    }

    /// Calculate merkle root from transaction hashes
    pub fn calculate_merkle_root(&self) -> String {
        if self.transactions.is_empty() {
            return "0".repeat(64);
        }

        let mut hashes: Vec<String> = self.transactions
            .iter()
            .map(|tx| tx.id.clone())
            .collect();

        while hashes.len() > 1 {
            let mut new_level = Vec::new();
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
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

    /// Calculate block hash
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.id, self.previous_hash, self.timestamp, self.nonce, self.merkle_root
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        let mut hash_string = String::new();
        for byte in result.iter() {
            write!(&mut hash_string, "{:02x}", byte).unwrap();
        }
        hash_string
    }

    /// Mine block with proof-of-work
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        println!("⛏️  Mining block {}...", self.id);

        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("✅ Block mined! Hash: {}", self.hash);
        println!("   Nonce: {}\n", self.nonce);
    }
}

/// Blockchain with UTXO set
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
    pub utxo_set: HashMap<String, Vec<TXOutput>>, // UTXO set for fast balance queries
}

impl Blockchain {
    /// Create new blockchain with genesis block
    pub fn new(difficulty: usize, genesis_address: &str) -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            difficulty,
            utxo_set: HashMap::new(),
        };

        // Create coinbase transaction for genesis block
        let coinbase = Transaction::new_coinbase(genesis_address, Some("Genesis Block".to_string()));

        let mut genesis = Block::new(0, String::from("0"), vec![coinbase.clone()]);
        genesis.mine_block(difficulty);

        // Initialize UTXO set with genesis outputs
        blockchain.utxo_set.insert(coinbase.id.clone(), coinbase.vout.clone());

        blockchain.blocks.push(genesis);
        blockchain
    }

    /// Add block with mining reward
    pub fn add_block(&mut self, transactions: Vec<Transaction>, miner_address: &str) {
        // Verify all transactions
        for tx in &transactions {
            if !tx.verify(&self.utxo_set) {
                panic!("❌ Invalid transaction detected!");
            }
        }

        // Create coinbase transaction (mining reward)
        let coinbase = Transaction::new_coinbase(miner_address, None);

        // Combine coinbase with other transactions
        let mut all_transactions = vec![coinbase.clone()];
        all_transactions.extend(transactions.clone());

        let previous_hash = self.get_latest_block().hash.clone();
        let id = self.blocks.len() as u64;

        let mut new_block = Block::new(id, previous_hash, all_transactions);
        new_block.mine_block(self.difficulty);

        // Update UTXO set
        self.update_utxo_set(&new_block);

        self.blocks.push(new_block);
    }

    /// Update UTXO set after adding a block
    fn update_utxo_set(&mut self, block: &Block) {
        for tx in &block.transactions {
            // Remove spent outputs
            if !tx.is_coinbase() {
                for input in &tx.vin {
                    if let Some(outputs) = self.utxo_set.get_mut(&input.txid) {
                        outputs.remove(input.vout);
                        if outputs.is_empty() {
                            self.utxo_set.remove(&input.txid);
                        }
                    }
                }
            }

            // Add new outputs
            self.utxo_set.insert(tx.id.clone(), tx.vout.clone());
        }
    }

    /// Get latest block
    pub fn get_latest_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    /// Get balance for an address
    pub fn get_balance(&self, address: &str) -> i32 {
        let mut balance = 0;

        for (_txid, outputs) in &self.utxo_set {
            for output in outputs {
                if output.can_be_unlocked_with(address) {
                    balance += output.value;
                }
            }
        }

        balance
    }

    /// Validate blockchain
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            // Check hash
            if current.hash != current.calculate_hash() {
                println!("❌ Invalid hash for block {}", current.id);
                return false;
            }

            // Check link
            if current.previous_hash != previous.hash {
                println!("❌ Invalid previous hash for block {}", current.id);
                return false;
            }

            // Check proof-of-work
            let target = "0".repeat(self.difficulty);
            if &current.hash[..self.difficulty] != target {
                println!("❌ Invalid proof-of-work for block {}", current.id);
                return false;
            }
        }

        println!("✅ Blockchain is valid!");
        true
    }

    /// Display blockchain
    pub fn display(&self) {
        println!("\n{}", "=".repeat(100));
        println!("BLOCKCHAIN WITH UTXO MODEL (Difficulty: {})", self.difficulty);
        println!("{}\n", "=".repeat(100));

        for block in &self.blocks {
            println!("--- Block {} ---", block.id);
            println!("Hash:          {}", block.hash);
            println!("Previous Hash: {}", block.previous_hash);
            println!("Timestamp:     {}", block.timestamp);
            println!("Nonce:         {}", block.nonce);
            println!("Merkle Root:   {}", block.merkle_root);
            println!("Transactions:  {}", block.transactions.len());

            for (idx, tx) in block.transactions.iter().enumerate() {
                if tx.is_coinbase() {
                    println!("  [{}] Coinbase -> {} gets {} coins", idx, tx.vout[0].pub_key_hash, tx.vout[0].value);
                } else {
                    println!("  [{}] Transaction {}", idx, &tx.id[..16]);
                    println!("      Inputs:  {}", tx.vin.len());
                    println!("      Outputs: {}", tx.vout.len());
                }
            }
            println!();
        }
    }
}

// ================================================================================================
// WALLET SYSTEM
// ================================================================================================

/// Simple wallet with public/private key pair
#[derive(Debug, Clone)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

impl Wallet {
    /// Create new wallet
    pub fn new(name: &str) -> Self {
        // In production, use real cryptographic key generation (ECDSA)
        // For education, we'll use deterministic keys based on name
        let private_key = format!("private_key_{}", name);
        let public_key = format!("public_key_{}", name);

        Wallet {
            private_key,
            public_key,
        }
    }

    /// Get wallet address (public key hash)
    pub fn get_address(&self) -> String {
        hash_pub_key(&self.public_key)
    }

    /// Sign data (simplified)
    pub fn sign(&self, data: &str) -> String {
        // In production, use ECDSA signing
        // For education, we'll create a simple signature
        format!("sig_{}_{}", self.private_key, hash_data(data))
    }
}

// ================================================================================================
// UTILITY FUNCTIONS
// ================================================================================================

/// Hash public key to create address
fn hash_pub_key(pub_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pub_key.as_bytes());
    let result = hasher.finalize();

    let mut hash_string = String::new();
    for byte in result.iter().take(20) { // Take first 20 bytes like Bitcoin
        write!(&mut hash_string, "{:02x}", byte).unwrap();
    }
    hash_string
}

/// Hash arbitrary data
fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();

    let mut hash_string = String::new();
    for byte in result.iter().take(8) {
        write!(&mut hash_string, "{:02x}", byte).unwrap();
    }
    hash_string
}

/// Find spendable outputs for a transaction
fn find_spendable_outputs(
    pub_key_hash: &str,
    amount: i32,
    utxo_set: &HashMap<String, Vec<TXOutput>>,
) -> (i32, HashMap<String, Vec<usize>>) {
    let mut accumulated = 0;
    let mut unspent_outputs: HashMap<String, Vec<usize>> = HashMap::new();

    for (txid, outputs) in utxo_set {
        for (idx, output) in outputs.iter().enumerate() {
            if output.can_be_unlocked_with(pub_key_hash) && accumulated < amount {
                accumulated += output.value;
                unspent_outputs
                    .entry(txid.clone())
                    .or_insert_with(Vec::new)
                    .push(idx);

                if accumulated >= amount {
                    return (accumulated, unspent_outputs);
                }
            }
        }
    }

    (accumulated, unspent_outputs)
}

// ================================================================================================
// MAIN DEMONSTRATION
// ================================================================================================

fn main() {
    println!("🔗 Blockchain 104: UTXO Model with Wallets\n");
    println!("📚 This demonstrates:");
    println!("   - UTXO (Unspent Transaction Output) model");
    println!("   - Wallet system with addresses");
    println!("   - Transaction inputs/outputs");
    println!("   - Balance tracking");
    println!("   - Mining rewards\n");

    // Create wallets
    println!("👛 Creating wallets...\n");
    let alice_wallet = Wallet::new("alice");
    let bob_wallet = Wallet::new("bob");
    let charlie_wallet = Wallet::new("charlie");
    let miner_wallet = Wallet::new("miner");

    println!("Alice's address:   {}", alice_wallet.get_address());
    println!("Bob's address:     {}", bob_wallet.get_address());
    println!("Charlie's address: {}", charlie_wallet.get_address());
    println!("Miner's address:   {}", miner_wallet.get_address());

    // Create blockchain with Alice getting genesis reward
    println!("\n⛓️  Creating blockchain...\n");
    let mut blockchain = Blockchain::new(3, &alice_wallet.get_address());

    println!("💰 Initial balances:");
    println!("   Alice:   {} coins", blockchain.get_balance(&alice_wallet.get_address()));
    println!("   Bob:     {} coins", blockchain.get_balance(&bob_wallet.get_address()));
    println!("   Charlie: {} coins", blockchain.get_balance(&charlie_wallet.get_address()));

    // Block 1: Alice sends 30 coins to Bob
    println!("\n📤 Block 1: Alice sends 30 coins to Bob...\n");
    let tx1 = Transaction::new_utxo_transaction(
        &alice_wallet,
        &bob_wallet.get_address(),
        30,
        &blockchain.utxo_set,
    ).expect("Failed to create transaction");

    blockchain.add_block(vec![tx1], &miner_wallet.get_address());

    println!("💰 Balances after Block 1:");
    println!("   Alice:   {} coins", blockchain.get_balance(&alice_wallet.get_address()));
    println!("   Bob:     {} coins", blockchain.get_balance(&bob_wallet.get_address()));
    println!("   Miner:   {} coins (mining reward)", blockchain.get_balance(&miner_wallet.get_address()));

    // Block 2: Bob sends 15 coins to Charlie
    println!("\n📤 Block 2: Bob sends 15 coins to Charlie...\n");
    let tx2 = Transaction::new_utxo_transaction(
        &bob_wallet,
        &charlie_wallet.get_address(),
        15,
        &blockchain.utxo_set,
    ).expect("Failed to create transaction");

    blockchain.add_block(vec![tx2], &miner_wallet.get_address());

    println!("💰 Balances after Block 2:");
    println!("   Alice:   {} coins", blockchain.get_balance(&alice_wallet.get_address()));
    println!("   Bob:     {} coins", blockchain.get_balance(&bob_wallet.get_address()));
    println!("   Charlie: {} coins", blockchain.get_balance(&charlie_wallet.get_address()));
    println!("   Miner:   {} coins", blockchain.get_balance(&miner_wallet.get_address()));

    // Block 3: Multiple transactions
    println!("\n📤 Block 3: Alice sends 10 to Charlie, Bob sends 5 to Alice...\n");
    let tx3a = Transaction::new_utxo_transaction(
        &alice_wallet,
        &charlie_wallet.get_address(),
        10,
        &blockchain.utxo_set,
    ).expect("Failed to create transaction");

    let tx3b = Transaction::new_utxo_transaction(
        &bob_wallet,
        &alice_wallet.get_address(),
        5,
        &blockchain.utxo_set,
    ).expect("Failed to create transaction");

    blockchain.add_block(vec![tx3a, tx3b], &miner_wallet.get_address());

    println!("💰 Final balances:");
    println!("   Alice:   {} coins", blockchain.get_balance(&alice_wallet.get_address()));
    println!("   Bob:     {} coins", blockchain.get_balance(&bob_wallet.get_address()));
    println!("   Charlie: {} coins", blockchain.get_balance(&charlie_wallet.get_address()));
    println!("   Miner:   {} coins", blockchain.get_balance(&miner_wallet.get_address()));

    // Display blockchain
    blockchain.display();

    // Validate blockchain
    println!("\n--- Validation ---\n");
    blockchain.is_chain_valid();

    // Show UTXO set
    println!("\n--- UTXO Set ({} transactions) ---", blockchain.utxo_set.len());
    for (txid, outputs) in &blockchain.utxo_set {
        println!("Transaction {}:", &txid[..16]);
        for (idx, output) in outputs.iter().enumerate() {
            println!("  Output[{}]: {} coins -> {}", idx, output.value, &output.pub_key_hash[..16]);
        }
    }

    println!("\n✨ Blockchain 104 demonstration complete!");
    println!("\n📖 Key Concepts Demonstrated:");
    println!("   ✅ UTXO model (inputs reference previous outputs)");
    println!("   ✅ Wallet addresses (hashed public keys)");
    println!("   ✅ Transaction verification");
    println!("   ✅ Balance calculation from UTXO set");
    println!("   ✅ Mining rewards (coinbase transactions)");
    println!("   ✅ Change outputs (when sending partial amounts)");
}

