# Hash Data

This Rust project demonstrates how to hash data using three different cryptographic hash functions: SHA-256, SHA-3 (SHA3-256), and BLAKE2b with 256-bit truncation. Each function produces a unique hash for a given input, commonly used in cryptographic applications, data integrity checks, and unique identifiers.

## Hash Functions Used
- SHA-256: Generates a 256-bit hash using the SHA-2 algorithm.
- SHA3-256: Generates a 256-bit hash using the SHA-3 algorithm.
- BLAKE2b-256: Uses the BLAKE2b hashing algorithm, truncated to a 256-bit (32-byte) output.

## Code Explanation
In the code, we can use different hash crates to generate each hash:

- sha2 for SHA-256
- sha3 for SHA3-256
- blake2 for BLAKE2b-256

Each function receives the input data as a byte slice (&[u8]), hashes it, and encodes the result in hexadecimal format.

```rust
use sha2::{Sha256, Digest};
use sha3::{Sha3_256};
use blake2::{Blake2b256, Digest as BlakeDigest};

fn hash_sha256(input: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}

fn hash_sha3(input: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}

fn hash_blake2(input: &[u8]) -> String {
    let mut hasher = Blake2b256::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}
```

## Tests

I’ve included tests to verify that each hashing function produces the expected output.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_sha256() {
        let data = b"Hello, world!";
        let expected_hash = "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3";
        let result = hash_sha256(data);
        assert_eq!(result, expected_hash);
    }

    #[test]
    fn test_hash_sha3() {
        let data = b"Hello, world!";
        let expected_hash = "f345a219da005ebe9c1a1eaad97bbf38a10c8473e41d0af7fb617caa0c6aa722";
        let result = hash_sha3(data);
        assert_eq!(result, expected_hash);
    }

    #[test]
    fn test_hash_blake2() {
        let data = b"Hello, world!";
        let expected_hash = "a2764d133a16816b5847a737a786f2ece4c148095c5faa73e24b4cc5d666c3e4";
        let result = hash_blake2(data);
        assert_eq!(result, expected_hash);
    }
}
```

## Explanation for Mismatched Hash Lengths

If you receive an assertion failure due to a mismatched hash length, it’s usually because of a mismatch between the expected and actual output length. For example, Blake2b512 generates a 512-bit hash by default, which can be truncated to 256 bits (32 bytes) if needed. To address this, we use Blake2b256, which natively supports 256-bit output, ensuring our tests match expected values accurately.

## Run tests

Run tests by following command 

```rust
cargo test
```

If all tests pass, you’ll see output similar to this:

```rust
running 3 tests
test tests::test_hash_sha256 ... ok
test tests::test_hash_sha3 ... ok
test tests::test_hash_blake2 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```






