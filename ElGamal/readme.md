# Project Documentation for ElGamal Encryption Library

ElGamal is a sophisticated asymmetric encryption algorithm built on the powerful principles of modular arithmetic and discrete logarithms. This library enables both encryption and decryption of messages with ElGamal, showcasing modular exponentiation and inverse operations. Let’s explore how to use this library from initialization to encryption and decryption.

## Code Structure

The library is organized as follows:

- ElGamal: This is the main struct encapsulating the algorithm, with attributes such as the prime number  𝑝 base 𝑔, private key, and public key.
- encrypt: This method allows users to encrypt a message with a given ephemeral key.
- decrypt: This method decrypts a ciphertext back to the original message.
- mod_inv: A specialized helper function to calculate the modular inverse.

## Installation Guide

Add the library to your project:
In your Cargo.toml, include:

```rust
[dependencies]
rand = "0.8"  # Or the latest version
```

## Initialize the ElGamal Struct:
Begin by creating an instance of ElGamal with the necessary parameters: a large prime 𝑝, a generator  𝑔, and your private key. The public key is derived automatically.

```rust
let elgamal = ElGamal::new(23, 5, 6);  // Example parameters
```

## Encrypt a Message:
Use the encrypt function by specifying the message and a randomly selected ephemeral key 𝑘. This function returns a tuple (c1, c2) representing the encrypted message.

```rust
let message = 12;
let k = 3; // Ephemeral key
let ciphertext = elgamal.encrypt(message, k);
```

## Decrypt a Ciphertext:
To decrypt, simply pass the ciphertext to decrypt. This method uses the private key to reconstruct the original message.

```rust
let decrypted_message = elgamal.decrypt(ciphertext);
```

Run cargo test to verify the functionality of each method. These tests ensure accuracy and reliability across encryption and decryption operations.

## Explanation of mod_inv

The mod_inv function calculates the modular inverse, a concept that’s pivotal in cryptographic operations. It ensures that division operations are performed securely within the confines of modular arithmetic, guaranteeing data integrity.

```rust
fn main() {
    let elgamal = ElGamal::new(23, 5, 6);  // Initialize with prime p, base g, and private key
    let message = 12;
    let k = 3;  // Random ephemeral key
    let ciphertext = elgamal.encrypt(message, k);
    let decrypted_message = elgamal.decrypt(ciphertext);

    println!("ElGamal Encryption: c1 = {}, c2 = {}", ciphertext.0, ciphertext.1);
    println!("ElGamal Decryption: message = {}", decrypted_message);
}
```

### Additional tips

- Prime Selection: Larger primes improve security but may impact performance.
- Random Ephemeral Key (k): It’s essential that this is chosen randomly each time for optimal security.
- Testing: Running the unit tests helps validate encryption consistency and detects potential vulnerabilities.
