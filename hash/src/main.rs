use sha2::{Sha256, Digest};
use sha3::Sha3_256;
use blake2::{Blake2b512, Digest as BlakeDigest};
use hex;

fn hash_sha256(input: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

fn hash_sha3(input: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

fn hash_blake2(input: &[u8]) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(&result[..32]) // Truncate to 256 bits (32 bytes)
}

fn main() {
    let data = b"Hello, world!";

    let sha256_hash = hash_sha256(data);
    println!("SHA-256: {}", sha256_hash);

    let sha3_hash = hash_sha3(data);
    println!("SHA-3: {}", sha3_hash);

    let blake2_hash = hash_blake2(data);
    println!("Blake2: {}", blake2_hash);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_sha256() {
        let data = b"Hello, world!";
        let result = hash_sha256(data);
        assert_eq!(result, "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3");
    }

    #[test]
    fn test_hash_sha3() {
        let data = b"Hello, world!";
        let result = hash_sha3(data);
        assert_eq!(result, "f345a219da005ebe9c1a1eaad97bbf38a10c8473e41d0af7fb617caa0c6aa722");
    }

    #[test]
    fn test_hash_blake2() {
        let data = b"Hello, world!";
        let expected_hash = "a2764d133a16816b5847a737a786f2ece4c148095c5faa73e24b4cc5d666c3e4";
        let result = hash_blake2(data);
        assert_eq!(result, expected_hash);
    }
}
