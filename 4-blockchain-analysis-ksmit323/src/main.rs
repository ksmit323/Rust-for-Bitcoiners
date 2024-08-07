use std::{env, time};

use bitcoincore_rpc::{
    bitcoin::{self, block, Amount, Block, BlockHash, Transaction},
    json::{self, GetBlockchainInfoResult},
    jsonrpc::{self, error::RpcError},
    Auth, Client, RpcApi,
};
use chrono::Duration;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        // let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        // let rpc_password: String =
            // env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        Client::new(&rpc_url, Auth::None).unwrap()
    };
}

// static client: Client = Client::new("url", Auth::UserPass("user".to_owned(), "password".to_owned())).unwrap();

// TODO: Task 1
fn time_to_mine(block_height: u64) -> Result<Duration, bitcoincore_rpc::Error> {
    // * is a deref operator which invokes the Deref trait of the type RPC_CLIENT which was created
    // when the lazy macro is expanded
    // if a value has a static lifetime then it means that value lives as long as the program lives
    let rpc_client: &Client = &*RPC_CLIENT;

    let block_hash = rpc_client.get_block_hash(block_height)?;
    let block = rpc_client.get_block(&block_hash)?;

    let prev_block_hash = block.header.prev_blockhash;
    let prev_block = rpc_client.get_block(&prev_block_hash)?;

    let time_diff = block.header.time - prev_block.header.time;
    let duration = Duration::seconds(time_diff as i64);

    Ok(duration)
}

// TODO: Task 2
fn number_of_transactions(block_height: u64) -> Result<u16, bitcoincore_rpc::Error> {
    let rpc_client: &Client = &*RPC_CLIENT;
    let block_hash = rpc_client.get_block_hash(block_height)?;
    let block = rpc_client.get_block(&block_hash)?;

    Ok(block.txdata.len() as u16)
}

// TODO: Adventure

fn get_block_details(
    rpc_client: &Client,
    block_height: u64,
) -> Result<Block, bitcoincore_rpc::Error> {
    let block_hash = rpc_client.get_block_hash(block_height)?;
    let block = rpc_client.get_block(&block_hash)?;

    Ok(block)
}

fn get_blockchain_info(
    rpc_client: &Client,
) -> Result<GetBlockchainInfoResult, bitcoincore_rpc::Error> {
    rpc_client.get_blockchain_info()
}

fn get_tx_details(
    rpc_client: &Client,
    txid: &bitcoin::Txid,
) -> Result<json::GetTransactionResult, bitcoincore_rpc::Error> {
    Ok(rpc_client.get_transaction(txid, None)?)
}

fn get_wallet_balance(rpc_client: &Client) -> Result<Amount, bitcoincore_rpc::Error> {
    rpc_client.get_balance(None, None)
}

fn get_block_difficulty(
    rpc_client: &Client,
    block_height: u64,
) -> Result<f64, bitcoincore_rpc::Error> {
    let block_hash = rpc_client.get_block_hash(block_height)?;
    let block_header = rpc_client.get_block_header(&block_hash)?;

    Ok(block_header.difficulty_float())
}

fn get_mempool_size(rpc_client: &Client) -> Result<usize, bitcoincore_rpc::Error> {
    let mempool = rpc_client.get_raw_mempool()?;

    Ok(mempool.len())
}

fn get_coinbase_tx(
    rpc_client: &Client,
    block_height: u64,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let block_hash = rpc_client.get_block_hash(block_height)?;
    let block = rpc_client.get_block(&block_hash)?;

    match block.coinbase() {
        Some(coinbase_tx) => Ok(coinbase_tx.clone()),
        None => Err("No tx's in block".into()),
    }
}

fn main() {
    // you can use rpc_client here as if it was a global variable
    // println!("{:?}", res);
    const TIMEOUT_UTXO_SET_SCANS: time::Duration = time::Duration::from_secs(60 * 8); // 8 minutes
    dotenv::dotenv().ok();
    let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
    let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
    let rpc_password: String =
        env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");

    let custom_timeout_transport = jsonrpc::simple_http::Builder::new()
        .url(&rpc_url)
        .expect("invalid rpc url")
        .auth(rpc_user, Some(rpc_password))
        .timeout(TIMEOUT_UTXO_SET_SCANS)
        .build();
    let custom_timeout_rpc_client =
        jsonrpc::client::Client::with_transport(custom_timeout_transport);

    let rpc_client = Client::from_jsonrpc(custom_timeout_rpc_client);
    let res: json::GetTxOutSetInfoResult =
        rpc_client.get_tx_out_set_info(None, None, None).unwrap();
    println!("{:?}", res);
}
