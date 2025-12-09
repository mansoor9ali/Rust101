// represent a block from a blockchain, using Rust structs

pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub txn_data: String,
    pub nonce: u64,
}


// blockchain can be represented
pub struct Blockchain<T> {
    pub blocks: Vec<T>,
}


// "let" keyword to assign a new value to the variable
fn main() {
    // Create a blockchain with 5 blocks
    let mut blockchain = Blockchain {
        blocks: Vec::new(),
    };

    // Genesis block (first block)
    let block1 = Block {
        id: 0,
        hash: String::from("0000000000000000000a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w"),
        previous_hash: String::from("0000000000000000000000000000000000000000000000000000000000000000"),
        timestamp: 1625247600,
        txn_data: String::from("Genesis Block"),
        nonce: 0,
    };

    // Block 2
    let block2 = Block {
        id: 1,
        hash: String::from("0000000000000000000b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x"),
        previous_hash: String::from("0000000000000000000a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w"),
        timestamp: 1625247660,
        txn_data: String::from("Alice pays Bob 10 BTC"),
        nonce: 2083236893,
    };

    // Block 3
    let block3 = Block {
        id: 2,
        hash: String::from("0000000000000000000c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y"),
        previous_hash: String::from("0000000000000000000b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x"),
        timestamp: 1625247720,
        txn_data: String::from("Bob pays Charlie 5 BTC"),
        nonce: 3094567821,
    };

    // Block 4
    let block4 = Block {
        id: 3,
        hash: String::from("0000000000000000000d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z"),
        previous_hash: String::from("0000000000000000000c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y"),
        timestamp: 1625247780,
        txn_data: String::from("Charlie pays David 3 BTC"),
        nonce: 4105678932,
    };

    // Block 5
    let block5 = Block {
        id: 4,
        hash: String::from("0000000000000000000e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a"),
        previous_hash: String::from("0000000000000000000d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z"),
        timestamp: 1625247840,
        txn_data: String::from("David pays Eve 7 BTC"),
        nonce: 5216789043,
    };

    // Add blocks to the blockchain
    blockchain.blocks.push(block1);
    blockchain.blocks.push(block2);
    blockchain.blocks.push(block3);
    blockchain.blocks.push(block4);
    blockchain.blocks.push(block5);

    // Print blockchain details
    println!("Blockchain initialized with {} blocks\n", blockchain.blocks.len());

    for (index, block) in blockchain.blocks.iter().enumerate() {
        println!("--- Block {} ---", index);
        println!("Block ID: {}", block.id);
        println!("Block Hash: {}", block.hash);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Timestamp: {}", block.timestamp);
        println!("Transaction Data: {}", block.txn_data);
        println!("Nonce: {}", block.nonce);
        println!();
    }
}