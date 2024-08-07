use bincode::serialize;
use bitcoin::address::{NetworkChecked, NetworkUnchecked};
use bitcoin::block::Header as BlockHeader;
use bitcoin::consensus::encode::deserialize;
use bitcoin::BlockHash;
use bitcoin::CompactTarget;
use bitcoin::TxMerkleNode;
use bitcoin::Txid;
use bitcoin::{Address, Network};
use bitcoin::{ScriptBuf, Transaction, TxIn, TxOut};
use serde_json::Value;
use std::str::FromStr;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

const MINER_ADDRESS: &str = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
const PREV_BLOCK_HASH: &str = "0000abcd00000000000000000000000000000000000000000000000000000000"; // some dummy hash
const DIFFICULTY_TARGET: &str = "000000ffff00000000000000000000000000000000000000000000000000000000"; // const DIFFICULTY_TARGET: u32 = 0x1d00ffff;
const BLOCK_REWARD: u64 = 3_125_000_000; // 3.125 BTC in satoshis
const MAX_WEIGHT: u64 = 4_000_000;

fn main() {
    let script_pubkey = get_script_pubkey_from_address(MINER_ADDRESS);

    // Fill up block with transactions up to max weight and get accumulated fees
    let (mut transactions, fees) = compile_txs_and_fees_from_mempool();

    // Add coinbase to beginning of transactions
    let coinbase_tx = create_coinbase_transaction(BLOCK_REWARD + fees, script_pubkey);
    transactions.insert(0, coinbase_tx); 

    let mined_block = mine_the_block(transactions);

    output_results(mined_block)
}

fn get_script_pubkey_from_address(address: &str) -> ScriptBuf {
    let address: Address<NetworkUnchecked> = address.parse().unwrap(); // rando address
    let address: Address<NetworkChecked> = address.require_network(Network::Bitcoin).unwrap();
    bitcoin::Address::script_pubkey(&address)
}

fn compile_txs_and_fees_from_mempool() -> (Vec<Transaction>, u64) {
    let mut total_weight = 0;
    let mut total_fees = 0;
    
    let mut transactions = Vec::new();

    for entry in fs::read_dir("mempool").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.file_name().unwrap() != "mempool.json" {
            let tx_str = fs::read_to_string(path).unwrap();
            let (bitcoin_tx, fee) = convert_json_to_bitcoin_transaction(&tx_str).unwrap();
            total_weight += u64::from(bitcoin_tx.weight());

            if total_weight <= MAX_WEIGHT {
                transactions.push(bitcoin_tx);
                total_fees += fee;
            }
        }
    }

    (transactions, total_fees)
}

fn convert_json_to_bitcoin_transaction(
    json: &str,
) -> Result<(Transaction, u64), Box<dyn std::error::Error>> {
    let v: Value = serde_json::from_str(json)?;

    // Extract the hex representation of the transaction
    let tx_hex = v["hex"].as_str().ok_or("Missing hex field")?;

    // Decode the hex string into a byte vector
    let tx_bytes = hex::decode(tx_hex)?;

    // Deserialize the byte vector into a bitcoin::Transaction
    let tx: Transaction = deserialize(&tx_bytes)?;

    // Extract out fee
    let fee = v["fee"].as_u64().ok_or("Missing fee field")?;

    Ok((tx, fee))
}

fn create_coinbase_transaction(reward: u64, script_pubkey: ScriptBuf) -> Transaction {
    let input = TxIn::default();
    let output = TxOut {
        value: bitcoin::Amount::from_sat(reward),
        script_pubkey,
    };

    Transaction {
        version: bitcoin::transaction::Version::ONE,
        lock_time: bitcoin::absolute::LockTime::Seconds(bitcoin::locktime::absolute::Time::MAX), // TODO: Fix this with something more legit
        input: vec![input],
        output: vec![output],
    }
}

fn mine_the_block(transactions: Vec<Transaction>) -> (BlockHeader, Vec<Txid>) {
    println!("Mining new block...");

    let tx_ids: Vec<Txid> = transactions.iter().map(|tx| tx.compute_txid()).collect();
    let merkle_root = compute_merkle_root(tx_ids.clone());

    let mut block_header = BlockHeader {
        version: bitcoin::block::Version::ONE,
        prev_blockhash: BlockHash::from_str(PREV_BLOCK_HASH).unwrap(),
        merkle_root,
        time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32, // Should be block timestamp, doesn't matter right now
        bits: CompactTarget::from_consensus(0x1d00ffff),
        nonce: 0,
    };

    let target = DIFFICULTY_TARGET.to_string();

    // Iterate with a difference nonce until the difficulty is satsified
    loop {
        let block_hash = block_header.block_hash().to_string();

        if block_hash <= target {
            println!("Block mined! Hash: {}", block_hash);
            break;
        }

        block_header.nonce += 1;

        // Let's do a progress counter for my own sanity check
        if block_header.nonce % 100000 == 0 {
            println!(
                "Nonce: {}, Current hash: {}",
                block_header.nonce, block_hash
            );
        }
    }
    println!("Final Nonce: {}", block_header.nonce);

    (block_header, tx_ids)
}

fn compute_merkle_root(tx_ids: Vec<Txid>) -> TxMerkleNode {
    /*
       Build the merkle tree from the transaction IDs.  Each ID has to be in the
       TxMerkleNode data type, however. So get Ids -> convert to TxMerkleNode -> hash to root
    */
    let merkle_root: TxMerkleNode =
        bitcoin::merkle_tree::calculate_root(tx_ids.into_iter().map(TxMerkleNode::from))
            .expect("Unable to compute merkle root");

    merkle_root
}

fn output_results(mined_block: (BlockHeader, Vec<Txid>)) {
    let (block_header, tx_ids) = mined_block;
    let mut output = String::new();

    let serialized_header = serialize(&block_header).unwrap();
    let block_header_hex = hex::encode(serialized_header);

    // Write the block header
    output.push_str(&format!("{}", block_header_hex));

    // Write the transaction IDs
    for tx_id in tx_ids {
        output.push_str(&format!("{}\n", tx_id));
    }

    fs::write("out.txt", output).unwrap();
}

// TODO: Validate txs
// fn validate_the_txs() {
//      Idk what to do here yet
// }
