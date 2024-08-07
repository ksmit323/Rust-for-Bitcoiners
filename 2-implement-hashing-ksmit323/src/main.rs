use std::convert::TryInto;

const BLOCK_SIZE: usize = 32; // Size of each block in bits
const HASH_SIZE: usize = 32; // Size of the hash code in bits

struct XorHasher {
    state: [u8; HASH_SIZE],
    block_count: usize,
}

impl XorHasher {
    fn new() -> Self {
        XorHasher {
            state: [0; HASH_SIZE],
            block_count: 0,
        }
    }

    fn update(&mut self, data: &[u8]) {
        let mut offset = 0;

        while offset < data.len() {
            let mut block = [0u8; BLOCK_SIZE];
            let remaining = data.len() - offset;
            let block_size = remaining.min(BLOCK_SIZE);

            block[..block_size].copy_from_slice(&data[offset..offset + block_size]);
            self.process_block(&block);

            offset += block_size;
            self.block_count += 1;
        }
    }

    fn finalize(self) -> [u8; HASH_SIZE] {
        self.state
    }

    fn process_block(&mut self, block: &[u8; BLOCK_SIZE]) {
        for i in 0..HASH_SIZE {
            // since we have HASH_SIZE == BLOCK_SIZE this is easy
            self.state[i] ^= block[i];

            // wrap to operations prevent overflow error
            self.state[i] = self.state[i].wrapping_shl(2) | self.state[i].wrapping_shr(6); // left shift and OR right shit
            self.state[i] = self.state[i].wrapping_add(block[i]); // add block
            self.state[i] = self.state[i] ^ block[i].wrapping_shr(3); // XOR with right shifted block
        }
    }
}

fn xor_hash(data: &[u8]) -> [u8; HASH_SIZE] {
    let mut hasher = XorHasher::new();
    hasher.update(data);
    hasher.finalize()
}

// fn xor_hash_attack(data: &[u8]) -> Vec<u8> {
//     let mut padded_data = Vec::new();
//     let r = BLOCK_SIZE - (data.len() % BLOCK_SIZE);

//     if r != 0 {
//         let padding = vec![0; r];
//         padded_data.extend_from_slice(data);
//         padded_data.extend(padding);
//     }
//     let mut matching_message = Vec::new();

//     for _ in 1..=3 {
//         matching_message.extend_from_slice(&padded_data);
//     }
//     matching_message
// }

fn xor_hash_attack(data: &[u8]) -> Vec<u8> {
    let mut padded_data = data.to_vec();
    let r = BLOCK_SIZE - (data.len() % BLOCK_SIZE);

    if r != 0 {
        let padding = vec![0; r];
        padded_data.extend(padding);
    }

    let mut attack_data = padded_data.clone();

    for i in 0..attack_data.len() {
        attack_data[i] = attack_data[i].wrapping_add(1);
        if xor_hash(&attack_data) == xor_hash(&padded_data) {
            return attack_data;
        }
        attack_data[i] = attack_data[i].wrapping_sub(1); 
    }

    attack_data

}


#[cfg(test)]
mod tests {

    use quickcheck::QuickCheck;

    use super::*;

    // #[test]
    // fn test_xor_attack() {
    //     fn prop(data: Vec<u8>) -> bool {
    //         xor_hash(&data) == xor_hash(&xor_hash_attack(&data))
    //     }
    //     QuickCheck::new().quickcheck(prop as fn(Vec<u8>) -> bool);
    // }

    #[test]
    fn attack_demo() {
        let data = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0];
        println!("{}", data.len());
        let attack = xor_hash_attack(&data);
        println!("{:?}", attack.len());
        println!("{:?}", xor_hash(&data));
        println!("{:?}", xor_hash(&attack));
    }
    
    #[test]
    fn test_known_hash() {
        let hash = xor_hash(b"Rust for Bitcoin");
        assert_eq!(hash.len(), HASH_SIZE);
        println!("Hash: {:?}", hash);
    }

    #[test]
    fn test_consistent_hashes() {
        let data = b"Ad Astra, per aspera";
        let hash = xor_hash(data);
        let hash_again = xor_hash(data);

        assert_eq!(hash, hash_again, "Hashes should be equal for the same input");
    }

    #[test]
    fn test_hash_with_small_variations(){
        let some_data = b"Ex nihilo, nihil fit";  
        let mut other_data = some_data.to_vec();

        // Change one byte
        other_data[0] ^= 0x01;

        let some_hash = xor_hash(some_data);
        let other_hash = xor_hash(&other_data);

        assert_ne!(some_hash, other_hash, "Hashes should differ even with only small variations");  
    }

    #[test]
    fn test_collision_resistant() {
        let some_data = b"Rust is gnarly";
        let other_data = b"Bitcoin is even gnarlier";

        let some_hash = xor_hash(some_data);
        let other_hash = xor_hash(other_data);

        assert_ne!(some_hash, other_hash, "Hashes should be different");
    }

    #[test]
    fn test_hash_length() {
        let hash = xor_hash(b"Ethereum is cooler than Bitcoin");

        assert_eq!(hash.len(), HASH_SIZE, "Hash's length should equal HASH_SIZE constant");
    }

    #[test]
    fn test_xor_attack() {
        let data = b"Take me to Mars";
        let hash = xor_hash(data);

        let attack_data = xor_hash_attack(data);
        let attack_hash = xor_hash(&attack_data);

        println!("Original Hash: {:?}", hash);
        println!("Attack Hash: {:?}", attack_hash);

        assert_eq!(hash, attack_hash, "Hash and attack hash should be the same");
    }

}

fn main() {
    println!("Hello, world!");
}
