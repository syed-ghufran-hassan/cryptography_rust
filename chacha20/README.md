# ChaCha20-RS

**ChaCha20-RS** is a Rust implementation of the ChaCha20 stream cipher, designed for secure and fast encryption. This library allows developers to easily integrate ChaCha20 into their applications for secure communication.

## Features
- Fast encryption and decryption.
- Secure random number generation.
- Simple and intuitive API.

## Installation
To use ChaCha20-RS in your Rust project, add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
chacha20 = "0.10.0-pre.2"
rand = "0.8"  # or the version you're using
```


## Usage
```rust
use chacha20::{ChaCha20, Key, Nonce};
use rand::{RngCore, SeedableRng};

fn main() {
    // Generate a random key and nonce
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32]; // ChaCha20 uses a 256-bit key
    rng.fill_bytes(&mut key);
    let nonce: Nonce = [0; 12]; // ChaCha20 uses a 96-bit nonce

    // Create a new ChaCha20 cipher instance
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
    
    // Data to encrypt
    let mut data = b"Hello, World!";
    
    // Encrypt the data
    cipher.apply_keystream(&mut data[..]);
    println!("Encrypted data: {:?}", data);
}
```
To decrypt the data, simply apply the keystream again:
```rust
// To decrypt, just apply the keystream again
cipher.apply_keystream(&mut data[..]);
println!("Decrypted data: {:?}", std::str::from_utf8(data).unwrap());
```

## API Reference

ChaCha20: The main cipher struct used for encryption and decryption.

### Methods:

```rust
new(key: &Key, nonce: &Nonce): Creates a new ChaCha20 instance.
apply_keystream(&mut data: &mut [u8]): Applies the keystream to the data for encryption or decryption.
```

To run the tests for this library, use the following command:

```rust
cargo test
```


