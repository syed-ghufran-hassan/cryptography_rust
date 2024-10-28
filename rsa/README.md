# RSA implementation in Rust

This program demonstrates basic RSA encryption and decryption using the rsa crate in Rust. It:

Generates a 2048-bit RSA key pair.
Encrypts a message (plaintext) using the public key.
Decrypts the encrypted message using the private key.
Contains unit tests to validate correct functionality.

## Imports and Padding Setup

```rust
use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
```

OsRng: Cryptographically secure random number generator.
PaddingScheme: Defines how data is padded before encryption. Here, we use OAEP padding with SHA-256.
RsaPrivateKey and RsaPublicKey: Key types for RSA encryption and decryption.
Sha256: Hashing algorithm used for padding (necessary for OAEP padding).

## rsa_encrypt_decrypt Function

This function performs the core encryption and decryption tasks.

```rust
fn rsa_encrypt_decrypt(plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    // Encrypt the plaintext
    let encrypted = public_key
        .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), &plaintext[..])
        .expect("encryption failed");

    // Decrypt the encrypted text
    let decrypted = private_key
        .decrypt(PaddingScheme::new_oaep::<Sha256>(), &encrypted)
        .expect("decryption failed");

    (encrypted, decrypted)
}
```

The plaintext input is the message to encrypt.
RsaPrivateKey::new(&mut rng, bits): Generates a private key with 2048 bits.
RsaPublicKey::from(&private_key): Extracts the public key from the private key.
encrypt and decrypt methods: Used to encrypt with the public key and decrypt with the private key.

## Unit tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rsa::errors::Error;

    #[test]
    fn test_encrypt_decrypt_valid_message() {
        let plaintext = b"RSA encryption test";
        let (encrypted, decrypted) = rsa_encrypt_decrypt(plaintext);

        assert_ne!(encrypted, plaintext); // Encrypted text should differ from plaintext
        assert_eq!(decrypted, plaintext); // Decrypted text should match plaintext
    }

    #[test]
    fn test_padding_scheme() {
        let _padding = PaddingScheme::new_oaep::<Sha256>();
    }

    #[test]
    fn test_invalid_decryption() {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        let plaintext = b"Different RSA test";

        let encrypted = public_key
            .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), &plaintext[..])
            .expect("encryption failed");

        let other_private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate key");
        let result: Result<Vec<u8>, Error> = other_private_key.decrypt(PaddingScheme::new_oaep::<Sha256>(), &encrypted);

        assert!(result.is_err(), "Expected an error during decryption with incorrect key");
    }

    #[test]
    fn test_empty_plaintext() {
        let plaintext = b"";
        let (_, decrypted) = rsa_encrypt_decrypt(plaintext);

        assert_eq!(decrypted, plaintext); // Decrypted text should match empty plaintext
    }
}
```
test_encrypt_decrypt_valid_message: Encrypts a sample message and verifies it decrypts back to the original.
test_padding_scheme: Ensures the padding scheme can be instantiated without errors.
test_invalid_decryption: Attempts decryption with a different private key, which should result in an error.
test_empty_plaintext: Tests encrypting and decrypting an empty message.

The main function demonstrates a sample RSA encryption and decryption.

```rust
fn main() {
    let plaintext = b"hello RSA world";
    let (encrypted, decrypted) = rsa_encrypt_decrypt(plaintext);
    println!(
        "RSA: {} -> Encrypted -> {:?} -> Decrypted -> {}",
        String::from_utf8_lossy(plaintext),
        encrypted,
        String::from_utf8_lossy(&decrypted)
    );
}
```

Encrypts the message "hello RSA world", prints both the encrypted and decrypted outputs.
String::from_utf8_lossy: Converts bytes to a UTF-8 string for readable output.

