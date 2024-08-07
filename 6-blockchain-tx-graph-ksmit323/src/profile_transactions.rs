use bitcoin::hash_types::Txid;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::env;

use super::graph::Graph;

lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap()
    };
}

fn build_transaction_graph(start_height: u64, end_height: u64) -> Graph<Txid> {
    // Every Transaction has a set of Inputs and outputs
    // Each Input refers to an output of some earlier transaction
    // We say a Transaction A funds Transaction B if an ouput of A is an input of B
    // Build a graph where nodes represents Txid and an edge (t1, t2) is in the graph
    // if the transaction t1 funds transaction t2
    let mut graph = Graph::new();

    for height in start_height..=end_height {
        let block_hash = RPC_CLIENT.get_block_hash(height).unwrap();
        let block = RPC_CLIENT.get_block(&block_hash).unwrap();

        for tx in block.txdata {
            graph.insert_vertex(tx.compute_txid());

            for input in &tx.input {
                graph.insert_edge(input.previous_output.txid, tx.compute_txid());
            }
        }
    }

    graph
}
