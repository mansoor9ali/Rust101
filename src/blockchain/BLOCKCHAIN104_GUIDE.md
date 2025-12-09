# 🔗 Blockchain 104 Guide: UTXO Model, Wallets & Digital Signatures

## Overview

This guide covers an advanced blockchain implementation that introduces the **UTXO (Unspent Transaction Output) model** used by Bitcoin, along with wallet management and transaction verification. This is a major architectural evolution from simple transaction tracking to a production-like blockchain system.

---

## 📚 Table of Contents

1. [What's New in Blockchain 104](#whats-new-in-blockchain-104)
2. [Core Concepts](#core-concepts)
3. [Data Structures](#data-structures)
4. [UTXO Model Explained](#utxo-model-explained)
5. [Implementation Details](#implementation-details)
6. [Usage Examples](#usage-examples)
7. [Running the Code](#running-the-code)
8. [Comparison with Previous Versions](#comparison-with-previous-versions)
9. [Advanced Topics](#advanced-topics)

---

## What's New in Blockchain 104

### 🎯 Major Additions

| Feature | Block103 | Block104 (This) |
|---------|----------|-----------------|
| Transaction Model | Simple transfers | ✅ UTXO (like Bitcoin) |
| Inputs/Outputs | ❌ | ✅ TXInput/TXOutput |
| Wallets | ❌ | ✅ Public/Private keys |
| Addresses | ❌ | ✅ Hashed public keys |
| Balance Tracking | Manual calculation | ✅ UTXO set |
| Mining Rewards | ❌ | ✅ Coinbase transactions |
| Change Outputs | ❌ | ✅ Automatic |
| Transaction Verification | Basic | ✅ Input/Output validation |

---

## Core Concepts

### 1. UTXO Model (Unspent Transaction Output)

**The fundamental shift:** Instead of tracking account balances, we track **unspent outputs**.

#### Traditional Account Model (NOT used):
```
Alice:   100 coins
Bob:     50 coins
Charlie: 25 coins
```

#### UTXO Model (Used by Bitcoin):
```
Unspent Outputs:
- TX1.out[0]: 100 coins → Alice's address
- TX2.out[0]: 50 coins  → Bob's address
- TX3.out[1]: 25 coins  → Charlie's address
```

**Key Insight:** Your balance = sum of all UTXOs you can unlock with your private key.

---

### 2. Transaction Anatomy

```
Transaction:
  Inputs:  [References to previous outputs being spent]
  Outputs: [New outputs that can be spent later]
```

#### Example: Alice sends 30 coins to Bob

**Before transaction:**
```
UTXO Set:
  TX0.out[0]: 50 coins → Alice
```

**Transaction:**
```
Inputs:
  - TX0.out[0] (spending Alice's 50 coins)
  - Signature: Proves Alice owns this output
  
Outputs:
  - out[0]: 30 coins → Bob
  - out[1]: 20 coins → Alice (change)
```

**After transaction:**
```
UTXO Set:
  TX1.out[0]: 30 coins → Bob
  TX1.out[1]: 20 coins → Alice
  
Note: TX0.out[0] is now spent (removed from UTXO set)
```

---

### 3. Wallet System

```rust
Wallet {
    private_key: "secret_key_alice"
    public_key:  "public_key_alice"
}
```

**Address = Hash(public_key)**

Why hash? 
- Shorter addresses
- Additional security layer
- Privacy (public key only revealed when spending)

---

### 4. Coinbase Transactions

**Mining Reward:** First transaction in every block

```rust
Coinbase Transaction:
  Inputs:  [] (no inputs - created from thin air)
  Outputs: [50 coins → miner's address]
```

This is how new coins enter circulation.

---

## Data Structures

### TXInput (Transaction Input)

```rust
pub struct TXInput {
    pub txid: String,        // Which transaction?
    pub vout: usize,         // Which output index?
    pub signature: String,   // Proof of ownership
    pub pub_key: String,     // Sender's public key
}
```

**Purpose:** References a previous output to spend it.

**Validation:** Must prove you own the output by providing:
1. Public key that hashes to the output's pub_key_hash
2. Valid signature

---

### TXOutput (Transaction Output)

```rust
pub struct TXOutput {
    pub value: i32,              // Amount of coins
    pub pub_key_hash: String,    // Who can spend this?
}
```

**Purpose:** Creates new "coins" that can be spent in future transactions.

**Locking:** Locked to an address (pub_key_hash). Only the owner of the corresponding private key can spend it.

---

### Transaction

```rust
pub struct Transaction {
    pub id: String,              // Transaction hash
    pub vin: Vec<TXInput>,       // Inputs (spending)
    pub vout: Vec<TXOutput>,     // Outputs (creating)
    pub timestamp: i64,
}
```

**Two types:**
1. **Coinbase:** No inputs, creates new coins (mining reward)
2. **Regular:** Has inputs (spending UTXOs) and outputs (creating new UTXOs)

---

### Block

```rust
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub merkle_root: String,
}
```

**Changes from Block103:**
- Transactions now use UTXO model (vin/vout)
- First transaction is always coinbase (mining reward)

---

### Blockchain

```rust
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
    pub utxo_set: HashMap<String, Vec<TXOutput>>,  // 🆕 UTXO set
}
```

**Key Addition:** `utxo_set` - Fast lookup of unspent outputs
- Key: Transaction ID
- Value: List of unspent outputs from that transaction

---

## UTXO Model Explained

### Why UTXO Instead of Account Balances?

| Account Model | UTXO Model (Bitcoin/Block104) |
|---------------|-------------------------------|
| Track balance per account | Track unspent outputs |
| `Alice: 100 coins` | `TX1.out[0]: 100 coins → Alice` |
| ❌ Double-spend risk if not careful | ✅ Inherently prevents double-spending |
| ❌ Must lock account during update | ✅ Parallel transaction processing |
| Simple to understand | More complex but more robust |

---

### Transaction Lifecycle

#### 1. Creating a Transaction

```rust
Transaction::new_utxo_transaction(
    from_wallet: &Wallet,
    to: &str,
    amount: i32,
    utxo_set: &HashMap<String, Vec<TXOutput>>,
)
```

**Steps:**
1. Find unspent outputs belonging to `from_wallet`
2. Accumulate until we have enough for `amount`
3. Create inputs referencing those outputs
4. Create output sending `amount` to recipient
5. Create change output if necessary
6. Sign the transaction

#### Example Code:

```rust
let alice_wallet = Wallet::new("alice");
let bob_address = bob_wallet.get_address();

// Alice sends 30 coins to Bob
let tx = Transaction::new_utxo_transaction(
    &alice_wallet,
    &bob_address,
    30,
    &blockchain.utxo_set,
)?;
```

---

#### 2. Verifying a Transaction

```rust
pub fn verify(&self, utxo_set: &HashMap<String, Vec<TXOutput>>) -> bool
```

**Validation checks:**
1. ✅ All inputs reference existing UTXOs
2. ✅ Signatures are valid (proves ownership)
3. ✅ Input amounts ≥ output amounts (no creating coins)
4. ✅ Outputs can be unlocked by public keys

```rust
if !tx.verify(&blockchain.utxo_set) {
    panic!("Invalid transaction!");
}
```

---

#### 3. Mining a Block

```rust
blockchain.add_block(vec![tx1, tx2], &miner_address);
```

**What happens:**
1. Create coinbase transaction (mining reward → miner)
2. Verify all transactions
3. Create block with coinbase + transactions
4. Mine block (proof-of-work)
5. **Update UTXO set:**
   - Remove spent outputs (from inputs)
   - Add new outputs (from all transactions)
6. Add block to chain

---

### UTXO Set Management

```rust
fn update_utxo_set(&mut self, block: &Block) {
    for tx in &block.transactions {
        // Remove spent outputs
        if !tx.is_coinbase() {
            for input in &tx.vin {
                utxo_set[input.txid].remove(input.vout);
            }
        }
        
        // Add new outputs
        utxo_set.insert(tx.id, tx.vout);
    }
}
```

**UTXO Set = Database of all unspent outputs**

Benefits:
- ✅ Fast balance lookups
- ✅ Fast transaction validation
- ✅ No need to scan entire blockchain

---

## Implementation Details

### 1. Creating a Wallet

```rust
pub fn new(name: &str) -> Self {
    let private_key = format!("private_key_{}", name);
    let public_key = format!("public_key_{}", name);
    
    Wallet { private_key, public_key }
}
```

**Production note:** Use real ECDSA (Elliptic Curve Digital Signature Algorithm) for cryptographic key generation. This is simplified for education.

**Address generation:**
```rust
pub fn get_address(&self) -> String {
    hash_pub_key(&self.public_key)
}
```

---

### 2. Coinbase Transaction (Mining Reward)

```rust
pub fn new_coinbase(to: &str, data: Option<String>) -> Self {
    let txout = TXOutput::new(50, to);  // 50 coins reward
    
    let txin = TXInput {
        txid: String::new(),      // No previous transaction
        vout: 0,
        signature: data.unwrap_or("Mining reward".to_string()),
        pub_key: String::new(),
    };

    // ... calculate hash ...
}
```

**Special properties:**
- No real inputs (coins created from nothing)
- First transaction in every block
- This is how new coins enter circulation

---

### 3. Finding Spendable Outputs

```rust
fn find_spendable_outputs(
    pub_key_hash: &str,
    amount: i32,
    utxo_set: &HashMap<String, Vec<TXOutput>>,
) -> (i32, HashMap<String, Vec<usize>>)
```

**Algorithm:**
1. Iterate through UTXO set
2. Find outputs locked to `pub_key_hash`
3. Accumulate until reaching `amount`
4. Return accumulated amount and output references

**Example:**
```
Need: 30 coins
UTXO Set:
  TX1.out[0]: 20 coins → Alice ✅
  TX1.out[1]: 15 coins → Alice ✅
  TX2.out[0]: 10 coins → Bob   ❌

Accumulated: 35 coins
Returns: (35, {TX1: [0, 1]})
```

---

### 4. Creating a UTXO Transaction

```rust
pub fn new_utxo_transaction(
    from_wallet: &Wallet,
    to: &str,
    amount: i32,
    utxo_set: &HashMap<String, Vec<TXOutput>>,
) -> Result<Self, String>
```

**Step-by-step:**

```rust
// 1. Get sender's address
let from_pub_key_hash = hash_pub_key(&from_wallet.public_key);

// 2. Find spendable outputs
let (accumulated, valid_outputs) = 
    find_spendable_outputs(&from_pub_key_hash, amount, utxo_set);

// 3. Check sufficient funds
if accumulated < amount {
    return Err("Not enough funds!");
}

// 4. Create inputs (spending UTXOs)
let mut inputs = vec![];
for (txid, outputs) in valid_outputs {
    for out_idx in outputs {
        let signature = from_wallet.sign(&txid);
        inputs.push(TXInput::new(txid, out_idx, signature, ...));
    }
}

// 5. Create outputs
let mut outputs = vec![TXOutput::new(amount, to)];

// 6. Add change output if needed
if accumulated > amount {
    let change = accumulated - amount;
    outputs.push(TXOutput::new(change, &from_wallet.get_address()));
}
```

---

### 5. Balance Calculation

```rust
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
```

**Simple:** Sum all UTXOs locked to this address.

---

## Usage Examples

### Example 1: Create Wallets

```rust
let alice_wallet = Wallet::new("alice");
let bob_wallet = Wallet::new("bob");

println!("Alice's address: {}", alice_wallet.get_address());
println!("Bob's address:   {}", bob_wallet.get_address());
```

**Output:**
```
Alice's address: 7c2a3b4e5f6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f
Bob's address:   1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b
```

---

### Example 2: Create Blockchain with Genesis

```rust
// Genesis block gives 50 coins to Alice
let mut blockchain = Blockchain::new(3, &alice_wallet.get_address());

println!("Alice's balance: {} coins", blockchain.get_balance(&alice_wallet.get_address()));
```

**Output:**
```
Alice's balance: 50 coins
```

---

### Example 3: Send Coins

```rust
// Alice sends 30 coins to Bob
let tx = Transaction::new_utxo_transaction(
    &alice_wallet,
    &bob_wallet.get_address(),
    30,
    &blockchain.utxo_set,
)?;

blockchain.add_block(vec![tx], &miner_wallet.get_address());
```

**What happens:**
1. Alice's 50 coin UTXO is spent
2. New UTXOs created:
   - 30 coins → Bob
   - 20 coins → Alice (change)
   - 50 coins → Miner (reward)

**Balances:**
```
Alice:  20 coins (change)
Bob:    30 coins
Miner:  50 coins (mining reward)
```

---

### Example 4: Check Balances

```rust
println!("Alice:  {} coins", blockchain.get_balance(&alice_wallet.get_address()));
println!("Bob:    {} coins", blockchain.get_balance(&bob_wallet.get_address()));
println!("Miner:  {} coins", blockchain.get_balance(&miner_wallet.get_address()));
```

---

### Example 5: View UTXO Set

```rust
for (txid, outputs) in &blockchain.utxo_set {
    println!("Transaction {}", &txid[..16]);
    for (idx, output) in outputs.iter().enumerate() {
        println!("  Output[{}]: {} coins", idx, output.value);
    }
}
```

**Output:**
```
Transaction abc123def456...
  Output[0]: 30 coins
  Output[1]: 20 coins
Transaction 789ghi012jkl...
  Output[0]: 50 coins
```

---

## Running the Code

### Prerequisites

Add to `Cargo.toml`:

```toml
[[bin]]
name = "blockchain104"
path = "src/blockchain/blockchain104.rs"

[dependencies]
sha2 = "0.10"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
```

### Compile and Run

```bash
# Run the demonstration
cargo run --bin blockchain104

# Build only
cargo build --bin blockchain104
```

---

### Expected Output

```
🔗 Blockchain 104: UTXO Model with Wallets

📚 This demonstrates:
   - UTXO (Unspent Transaction Output) model
   - Wallet system with addresses
   - Transaction inputs/outputs
   - Balance tracking
   - Mining rewards

👛 Creating wallets...

Alice's address:   7c2a3b4e5f6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f
Bob's address:     1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b
Charlie's address: 9d8e7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e
Miner's address:   2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c

⛓️  Creating blockchain...

Mining block 0...
✅ Block mined! Hash: 000a1b2c3d4e5f...
   Nonce: 1234

💰 Initial balances:
   Alice:   50 coins
   Bob:     0 coins
   Charlie: 0 coins

📤 Block 1: Alice sends 30 coins to Bob...

⛏️  Mining block 1...
✅ Block mined! Hash: 000b2c3d4e5f6a...
   Nonce: 5678

💰 Balances after Block 1:
   Alice:   20 coins
   Bob:     30 coins
   Miner:   50 coins (mining reward)

...

✅ Blockchain is valid!

✨ Blockchain 104 demonstration complete!
```

---

## Comparison with Previous Versions

| Feature | Block101 | Block102 | Block103 | Block104 (This) |
|---------|----------|----------|----------|-----------------|
| **Basic Blocks** | ✅ | ✅ | ✅ | ✅ |
| **Mining (PoW)** | ❌ | ✅ | ✅ | ✅ |
| **Chain Validation** | ❌ | ✅ | ✅ | ✅ |
| **Transactions** | ❌ | String | Objects | ✅ UTXO Model |
| **Merkle Trees** | ❌ | ❌ | ✅ | ✅ |
| **Transaction Model** | ❌ | Simple | Simple | ✅ Inputs/Outputs |
| **Wallets** | ❌ | ❌ | ❌ | ✅ Public/Private keys |
| **Addresses** | ❌ | ❌ | ❌ | ✅ Hashed pub keys |
| **Balance Tracking** | ❌ | ❌ | Manual | ✅ UTXO Set |
| **Mining Rewards** | ❌ | ❌ | ❌ | ✅ Coinbase tx |
| **Change Outputs** | ❌ | ❌ | ❌ | ✅ Automatic |
| **Signature Verification** | ❌ | ❌ | ❌ | ✅ Basic |
| **Bitcoin-like** | ❌ | ❌ | ❌ | ✅ Yes! |

---

## Advanced Topics

### 1. Double-Spend Prevention

**How UTXO prevents double-spending:**

```
Transaction 1 (valid):
  Input: TX0.out[0] (Alice's 50 coins)
  Output: 30 coins → Bob

Transaction 2 (INVALID - double spend):
  Input: TX0.out[0] (Alice's 50 coins) ← Already spent!
  Output: 40 coins → Charlie
```

**Validation fails:** TX0.out[0] is no longer in UTXO set (it was removed when TX1 was processed).

---

### 2. UTXO Set vs. Scanning Blockchain

| Approach | Method | Speed |
|----------|--------|-------|
| **Scanning** | Iterate all blocks to find UTXOs | ❌ O(n) - slow |
| **UTXO Set** | Lookup in HashMap | ✅ O(1) - fast |

**Production:** Bitcoin Core maintains a UTXO set database (~5GB) for fast lookups.

---

### 3. Transaction Fees (Not Implemented Yet)

In production blockchains:

```
Fee = Sum(inputs) - Sum(outputs)
```

**Example:**
```
Inputs:  50 coins
Outputs: 30 (to Bob) + 19 (change) = 49 coins
Fee:     1 coin (goes to miner)
```

Miners prioritize transactions with higher fees.

---

### 4. SPV (Simplified Payment Verification)

With UTXO + Merkle trees:

**Light clients can verify transactions without downloading full blockchain:**
1. Download block headers only (~80 bytes each)
2. Request merkle proof for specific transaction
3. Verify transaction is in block
4. Check if UTXO is unspent

**Bandwidth saved:** ~99.9% (only need ~5 MB instead of ~500 GB for Bitcoin)

---

### 5. Upgrading to Production

**What's still needed:**

#### Real Cryptography
```rust
// Replace simplified signing with:
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
```

#### Persistence
```rust
// Use database instead of in-memory
use sled::Db;
let db = sled::open("blockchain_data")?;
```

#### Networking
```rust
// P2P network for block propagation
use tokio::net::{TcpListener, TcpStream};
```

#### Consensus
- Handle chain forks
- Longest chain rule
- Network synchronization

---

## Security Considerations

### ✅ What Block104 Provides

1. **UTXO Model**: Prevents double-spending at structural level
2. **Input Validation**: All inputs must reference existing UTXOs
3. **Output Validation**: Can't create more coins than inputs
4. **Balance Integrity**: Balance = sum of UTXOs (verifiable)

### ⚠️ Still Needed for Production

1. **Real ECDSA Signatures**: Current implementation is simplified
2. **Proper Key Management**: Store private keys securely
3. **Network Security**: P2P protocol security
4. **51% Attack Prevention**: Consensus mechanisms
5. **Transaction Pool**: Mempool management
6. **Script System**: Programmable locking (like Bitcoin Script)

---

## Learning Path

### You Are Here: Block104 ✅
- [x] Basic blockchain structure
- [x] Proof-of-Work mining
- [x] Transaction objects
- [x] Merkle trees
- [x] **UTXO model**
- [x] **Wallets with addresses**
- [x] **Transaction inputs/outputs**
- [x] **Mining rewards**

### Next Steps: Production Features

1. **Real Cryptography**
   - ECDSA signing/verification
   - Proper key generation
   - Address encoding (Base58)

2. **Persistence**
   - Database storage (sled/RocksDB)
   - UTXO set persistence
   - Blockchain state recovery

3. **Networking**
   - P2P protocol
   - Block propagation
   - Transaction broadcasting

4. **Advanced Features**
   - Transaction fees
   - Memory pool
   - Script system
   - Multi-signature

---

## Exercises

### Beginner
1. Create 5 different wallets and transfer coins between them
2. Calculate total supply (sum of all UTXOs)
3. Verify that input sum equals output sum for all transactions

### Intermediate
1. Implement transaction fees
2. Add transaction history lookup for an address
3. Create a function to export/import UTXO set

### Advanced
1. Implement real ECDSA signatures using `ring` crate
2. Add database persistence with `sled`
3. Create a REST API for wallet operations
4. Implement a simple script system for programmable locks

---

## Summary

### Key Achievements in Block104

✅ **UTXO Model**: Bitcoin-like transaction model
✅ **Wallets**: Public/private key pairs with addresses
✅ **Transaction Verification**: Input/output validation
✅ **Mining Rewards**: Coinbase transactions
✅ **Balance Tracking**: Fast UTXO set lookups
✅ **Change Outputs**: Automatic change calculation
✅ **Production Architecture**: Structure similar to real blockchains

### Architectural Evolution

```
Block101: Basic Blocks
    ↓
Block102: + Mining
    ↓
Block103: + Transactions + Merkle Trees
    ↓
Block104: + UTXO Model + Wallets (← You are here!)
    ↓
Production: + Real Crypto + Networking + Persistence
```

### Key Takeaways

1. **UTXO Model** is more robust than account balances for blockchain
2. **Transactions reference previous outputs**, creating an unbroken chain
3. **UTXO Set** enables fast validation without scanning entire blockchain
4. **Mining rewards** (coinbase) are how new coins enter circulation
5. **Change outputs** are necessary when spending more than needed

---

**File**: `blockchain104.rs`  
**Title**: Rust Blockchain Learning Series  
**Author**: Mansoor Ali Syed
**Date**: December 2025  
**Version**: 1.0.0 - UTXO Model Implementation

🎉 **Congratulations! You now understand Bitcoin's core architecture!** 🔗

---

## References

- **Bitcoin Whitepaper**: Satoshi Nakamoto's original UTXO design
- **Mastering Bitcoin**: Andreas Antonopoulos (Chapter 6: Transactions)
- **Bitcoin Core**: Reference implementation of UTXO model
- **Chapter 3 Reference**: Advanced production implementation

---

## FAQ

**Q: Why UTXO instead of account balances?**
A: UTXO prevents double-spending at the structural level, enables parallel processing, and simplifies verification.

**Q: What happens to spent outputs?**
A: They're removed from the UTXO set. Only unspent outputs are kept in memory.

**Q: How does a wallet know its balance?**
A: Sum all UTXOs in the UTXO set that are locked to that wallet's address.

**Q: Can I spend part of a UTXO?**
A: No! You must spend the entire UTXO and create a "change" output back to yourself.

**Q: How are new coins created?**
A: Only through coinbase transactions (mining rewards). No other way to create coins.

**Q: What prevents me from creating a transaction with more outputs than inputs?**
A: The verification function checks that sum(outputs) ≤ sum(inputs).

---

🚀 **Ready to build a production blockchain? Check out the Chapter 3 reference implementation!**

