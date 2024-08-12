use bitcoin::block::Header as BlockHeader;
use bitcoin::{BlockHash, CompactTarget, TxMerkleNode};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

pub const PREV_BLOCK_HASH: &str =
    "0000abcd00000000000000000000000000000000000000000000000000000000";
pub const DIFFICULTY_TARGET: &str =
    "00000ffff00000000000000000000000000000000000000000000000000000000";

pub fn create_block_header(tx_ids: Vec<bitcoin::Txid>) -> BlockHeader {
    BlockHeader {
        version: bitcoin::block::Version::ONE,
        prev_blockhash: BlockHash::from_str(PREV_BLOCK_HASH).unwrap(),
        merkle_root: compute_merkle_root(tx_ids),
        time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32,
        bits: CompactTarget::from_consensus(0x1d00ffff),
        nonce: 0,
    }
}

fn compute_merkle_root(tx_ids: Vec<bitcoin::Txid>) -> TxMerkleNode {
    /*
       Build the merkle tree from the transaction IDs.  Each ID has to be in the
       TxMerkleNode data type, however. So get Ids -> convert to TxMerkleNode -> hash to root
    */
    bitcoin::merkle_tree::calculate_root(tx_ids.into_iter().map(TxMerkleNode::from))
        .expect("Unable to compute merkle root")
}
