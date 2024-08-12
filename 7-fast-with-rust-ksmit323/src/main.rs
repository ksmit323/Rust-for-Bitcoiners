mod block;
mod mining;
mod transactions;
mod utils;

use crate::mining::mine_the_block;
use crate::transactions::{compile_txs_and_fees_from_mempool, create_coinbase_transaction};
use crate::utils::{get_script_pubkey_from_address, output_results};

const MINER_ADDRESS: &str = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"; // random address
const BLOCK_REWARD: u64 = 3_125_000_000; // 3.125 BTC in satoshis

fn main() {
    let script_pubkey = get_script_pubkey_from_address(MINER_ADDRESS);

    let (mut transactions, fees) = compile_txs_and_fees_from_mempool();

    let coinbase_tx = create_coinbase_transaction(BLOCK_REWARD + fees, script_pubkey);
    transactions.insert(0, coinbase_tx);

    if let Some(mined_block) = mine_the_block(transactions) {
        output_results(mined_block)
    }
}
