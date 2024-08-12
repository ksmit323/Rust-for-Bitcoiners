use bincode::serialize;
use bitcoin::address::{NetworkChecked, NetworkUnchecked};
use bitcoin::block::Header as BlockHeader;
use bitcoin::{Address, Network, ScriptBuf};
use std::fs;

pub fn get_script_pubkey_from_address(address: &str) -> ScriptBuf {
    let address: Address<NetworkUnchecked> = address.parse().unwrap();
    let address: Address<NetworkChecked> = address.require_network(Network::Bitcoin).unwrap();
    bitcoin::Address::script_pubkey(&address)
}

pub fn output_results(mined_block: (BlockHeader, Vec<bitcoin::Txid>)) {
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
