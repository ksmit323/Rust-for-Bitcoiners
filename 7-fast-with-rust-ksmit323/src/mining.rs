use crate::block::{create_block_header, DIFFICULTY_TARGET};
use bitcoin::block::Header as BlockHeader;
use bitcoin::Txid;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub fn mine_the_block(transactions: Vec<bitcoin::Transaction>) -> Option<(BlockHeader, Vec<Txid>)> {
    let target = DIFFICULTY_TARGET.to_string();

    // Setup block header
    let tx_ids: Vec<Txid> = transactions.iter().map(|tx| tx.compute_txid()).collect();
    let block_header = create_block_header(tx_ids.clone());

    let start_sequential = Instant::now();
    mine_block_sequentially(block_header, target.clone());
    let end_sequential = start_sequential.elapsed();

    let start_parallel = Instant::now();
    let result = mine_block_in_parallel(block_header, target, tx_ids);
    let end_parallel = start_parallel.elapsed();

    println!("Sequential Time: {:?}", end_sequential);
    println!("Parallel Time: {:?}", end_parallel);

    result
}

fn mine_block_sequentially(mut block_header: BlockHeader, target: String) {
    println!("Mining the block sequentially...");
    loop {
        let block_hash = block_header.block_hash().to_string();

        if block_hash <= target {
            break;
        }
        block_header.nonce += 1;

        // Progress counter
        if block_header.nonce % 100000 == 0 {
            println!("Nonce: {}", block_header.nonce);
        }
    }
}

fn mine_block_in_parallel(
    block_header: BlockHeader,
    target: String,
    tx_ids: Vec<Txid>,
) -> Option<(BlockHeader, Vec<Txid>)> {
    println!("Mining the block in parallel...");

    let nonce = Arc::new(AtomicU32::new(0));
    let found_target = Arc::new(AtomicBool::new(false));

    let num_threads = num_cpus::get();
    println!("Number of Threads: {}", num_threads);

    let result = (0..num_threads).into_par_iter().find_map_any(|_| {
        let mut local_header = block_header.clone();

        while !found_target.load(Ordering::Relaxed) {
            local_header.nonce = nonce.fetch_add(1, Ordering::Relaxed);
            let block_hash = local_header.block_hash().to_string();

            if block_hash <= target {
                println!("Block mined! Hash: {}", block_hash);
                found_target.store(true, Ordering::Relaxed);
                return Some((local_header, tx_ids.clone()));
            }

            // Progress counter
            if local_header.nonce % 100_000 == 0 && local_header.nonce != 0 {
                println!("Nonce: {}", local_header.nonce);
            }
        }
        None
    });
    println!("Final Nonce: {}", nonce.load(Ordering::Relaxed));

    result
}
