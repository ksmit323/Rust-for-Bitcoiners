#![allow(unused)]

use bs58;
use ripemd::Ripemd160;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::LinkedList as List;

#[derive(Serialize, Deserialize)]
pub struct BlockChain {
    pub blocks: List<Block>,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    hash: String,
    id: u128,
    transactions: List<Transaction>,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    txid: String,
}
#[derive(Serialize, Deserialize)]
pub struct TxIn {
    prev_txid: String,
    out: usize,
    signature: String, // to spend the output
}

#[derive(Serialize, Deserialize)]
pub struct TxOut {
    public_address: String,
    satoshis: u64,
    // 1 btc = 10^8 satoshis, in total 10^8 * 21 * 10^6 = 2.1 * 10^15
    // maximum value of u64 is greater than 10^19
    // so u64 is enough to store all valid satoshis
}

// Try to include bitcoin related functionalities like serialization, computing addresses etc.,
// You can add your own methods for different types and associated unit tests

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: List::new(),
        }
    }

    pub fn add_new_block(&mut self, block: Block) {
        self.blocks.push_back(block)
    }

    pub fn blockchain_length(&self) -> u128 {
        self.blocks.len().try_into().unwrap()
    }

    pub fn get_block_by_height(&self, index: usize) -> Option<&Block> {
        self.blocks.iter().nth(index)
    }

    pub fn compute_address(&self, public_key: &str) -> String {
        // Perform hashes
        let sha_hash_public_key = Sha256::digest(public_key.as_bytes());
        let ripemd160_hash_the_sha_hash = Ripemd160::digest(sha_hash_public_key);

        // Prepend version byte
        let mut address_bytes = vec![0x00];
        address_bytes.extend_from_slice(&ripemd160_hash_the_sha_hash);

        // Create and append checsum
        let double_sha256_checksum = &Sha256::digest(Sha256::digest(&address_bytes))[..4];
        address_bytes.extend_from_slice(double_sha256_checksum);

        // Encode to base58
        bs58::encode(address_bytes).into_string()
    }
}

impl Block {
    pub fn new(hash: String, id: u128) -> Self {
        Block {
            hash,
            id,
            transactions: List::new(),
        }
    }

    pub fn add_new_transaction(&mut self, transaction: Transaction) {
        self.transactions.push_back(transaction)
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn deserialize(serialized: &str) -> Self {
        serde_json::from_str(serialized).unwrap()
    }
}

impl Transaction {
    pub fn new(txid: String) -> Self {
        Transaction {
            inputs: List::new(),
            outputs: List::new(),
            txid,
        }
    }

    pub fn add_new_input(&mut self, input: TxIn) {
        self.inputs.push_back(input)
    }

    pub fn add_new_output(&mut self, output: TxOut) {
        self.outputs.push_back(output)
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn deserialize(serialized: &str) -> Self {
        serde_json::from_str(serialized).unwrap()
    }
}

impl TxIn {
    pub fn new(prev_txid: String, out: usize, signature: String) -> Self {
        TxIn {
            prev_txid,
            out,
            signature,
        }
    }

    pub fn is_valid_signature(&self) -> bool {
        // TODO: Need some logic here to validate the sig
        !self.signature.is_empty() // Just something here as I'm not sure on the logic yet
    }
}

impl TxOut {
    pub fn new(public_address: String, satoshis: u64) -> Self {
        TxOut {
            public_address,
            satoshis,
        }
    }
}
