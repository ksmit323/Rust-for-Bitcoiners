use bitcoin::{ScriptBuf, Transaction, TxIn, TxOut};
use serde_json::Value;
use std::fs;

const MAX_WEIGHT: u64 = 4_000_000;

pub fn compile_txs_and_fees_from_mempool() -> (Vec<Transaction>, u64) {
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

pub fn create_coinbase_transaction(reward: u64, script_pubkey: ScriptBuf) -> Transaction {
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

fn convert_json_to_bitcoin_transaction(
    json: &str,
) -> Result<(Transaction, u64), Box<dyn std::error::Error>> {
    let v: Value = serde_json::from_str(json)?;

    // Extract the hex representation of the transaction
    let tx_hex = v["hex"].as_str().ok_or("Missing hex field")?;

    // Decode the hex string into a byte vector
    let tx_bytes = hex::decode(tx_hex)?;

    // Deserialize the byte vector into a bitcoin::Transaction
    let tx: Transaction = bitcoin::consensus::deserialize(&tx_bytes)?;

    // Extract out fee
    let fee = v["fee"].as_u64().ok_or("Missing fee field")?;

    Ok((tx, fee))
}

// TODO: Validate txs
// fn validate_the_txs() {
//      Idk what to do here yet
// }