mod block;
mod linked_list;
mod mresult;

use block::*;
use std::io::{self, Write};

fn main() {
    let mut blockchain = BlockChain::new();

    //*  Doing a loop to just query a user for blocks or txs
    //*  I have no clue on creating event listeners or broadcasting to a network
    
    loop {
        // Prompt user
        println!("\nSimulate a new Blockchain");
        println!("1. Add a new block");
        println!("2. Add a transaction to the latest block");
        println!("3. Compute address from public key");
        println!("4. Exit");
        print!("Choose an option: ");

        // Get user input
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        // Match user's choice
        match choice.trim() {
            "1" => add_new_block(&mut blockchain),
            "2" => add_new_transaction(&mut blockchain),
            "3" => compute_address(&blockchain),
            "4" => {
                println!("Goodbye!");
                break
            },
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn add_new_block(blockchain: &mut BlockChain) {
    println!("Adding new block...");

    // TODO: Properly compute the hash

    let new_block = Block::new(
        String::from("Some hash here"),
        blockchain.blockchain_length(),
    );

    blockchain.add_new_block(new_block);

    println!("Block has been added")
}

fn add_new_transaction(blockchain: &mut BlockChain) {
    println!("Adding new tx...");

    if blockchain.blockchain_length() < 1 {
        println!("no blocks in the chain, dude");
        return;
    }

    // TODO: Properly compute Transaction data

    let new_tx = Transaction::new("Some tx id, idk".to_string());
    let latest_block = blockchain.blocks.back_mut().unwrap();
    latest_block.add_new_transaction(new_tx);

    println!("Added new transaction to latest block")
}

fn compute_address(blockchain: &BlockChain) {
    // Prompt user
    println!("Enter a public key:");
    let mut public_key = String::new();
    io::stdin().read_line(&mut public_key).unwrap();
    
    // Compute address from public key
    let address = blockchain.compute_address(public_key.trim());
    println!("Computed address: {}", address);
}
