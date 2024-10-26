extern crate aes;           // AES crate for symmetric encryption algorithms
extern crate block_modes;    // Crate to handle block cipher modes (like CBC)
extern crate hex_literal;    // Crate to handle hex literals for keys and IVs

use aes::Aes128;             // Using AES-128 from the aes crate
use block_modes::{BlockMode, Cbc}; // Import block cipher mode trait and CBC mode
use block_modes::block_padding::Pkcs7; // PKCS7 padding scheme for block cipher
use hex_literal::hex;        // Hex literal for key and IV encoding

// Create a type alias for AES-128 in CBC mode with PKCS7 padding
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

/// Function to encrypt the given plaintext using AES-128 in CBC mode
/// 
/// # Parameters:
/// - `key`: The secret key used for AES-128 encryption (16 bytes).
/// - `iv`: The initialization vector (IV) for CBC mode (16 bytes).
/// - `plaintext`: The plaintext message to be encrypted.
///
/// # Returns:
/// A `Vec<u8>` containing the ciphertext (encrypted data).
pub fn encrypt_aes128(key: &[u8], iv: &[u8], plaintext: &str) -> Vec<u8> {
    // Create a cipher instance using `new_from_slices` (Changed from `new_var`)
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap(); 
    // Encrypt the plaintext and return the ciphertext as a vector of bytes
    cipher.encrypt_vec(plaintext.as_bytes())
}

/// Function to decrypt the given ciphertext using AES-128 in CBC mode
/// 
/// # Parameters:
/// - `key`: The secret key used for AES-128 decryption (16 bytes).
/// - `iv`: The initialization vector (IV) for CBC mode (16 bytes).
/// - `ciphertext`: The ciphertext (encrypted data) to be decrypted.
///
/// # Returns:
/// A `String` containing the decrypted plaintext message.
pub fn decrypt_aes128(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> String {
    // Create a cipher instance using `new_from_slices` (Changed from `new_var`)
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap(); 
    // Decrypt the ciphertext and return the decrypted plaintext as a String
    let decrypted_ciphertext = cipher.decrypt_vec(ciphertext).unwrap();
    String::from_utf8(decrypted_ciphertext).unwrap()
}

fn main() {
    // Example 16-byte key and IV for AES-128 (you should use random values in practice)
    let key = hex!("000102030405060708090a0b0c0d0e0f"); // Secret key (16 bytes)
    let iv = hex!("101112131415161718191a1b1c1d1e1f");  // Initialization vector (IV, 16 bytes)

    // The plaintext message we want to encrypt
    let plaintext = "Hello, world! AES-128 CBC mode encryption";

    // Encrypt the plaintext
    let ciphertext = encrypt_aes128(&key, &iv, plaintext);
    println!("Ciphertext: {:?}", hex::encode(&ciphertext)); // Print ciphertext in hex format

    // Decrypt the ciphertext
    let decrypted_text = decrypt_aes128(&key, &iv, &ciphertext);
    println!("Decrypted text: {}", decrypted_text); // Print the decrypted plaintext
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes128_encryption_decryption() {
        // Example key and IV for AES-128 encryption
        let key = hex!("000102030405060708090a0b0c0d0e0f");
        let iv = hex!("101112131415161718191a1b1c1d1e1f");

        // Test data
        let plaintext = "Test encryption and decryption";

        // Encrypt the plaintext
        let ciphertext = encrypt_aes128(&key, &iv, plaintext);

        // Decrypt the ciphertext
        let decrypted_text = decrypt_aes128(&key, &iv, &ciphertext);

        // Assert that the decrypted text matches the original plaintext
        assert_eq!(plaintext, decrypted_text, "Decryption failed: original plaintext does not match decrypted text");
    }

    #[test]
    fn test_empty_string_encryption_decryption() {
        // Example key and IV for AES-128 encryption
        let key = hex!("000102030405060708090a0b0c0d0e0f");
        let iv = hex!("101112131415161718191a1b1c1d1e1f");

        // Test data: empty string
        let plaintext = "";

        // Encrypt the plaintext
        let ciphertext = encrypt_aes128(&key, &iv, plaintext);

        // Decrypt the ciphertext
        let decrypted_text = decrypt_aes128(&key, &iv, &ciphertext);

        // Assert that the decrypted text matches the original plaintext
        assert_eq!(plaintext, decrypted_text, "Decryption failed for empty string");
    }

    #[test]
    fn test_aes128_encryption_different_inputs() {
        // Example key and IV for AES-128 encryption
        let key = hex!("00112233445566778899aabbccddeeff");
        let iv = hex!("0123456789abcdef0123456789abcdef");

        // Different test inputs
        let inputs = vec![
            "A quick brown fox jumps over the lazy dog.",
            "Rust programming language is awesome.",
            "Cryptography in Rust using AES-128 CBC mode.",
        ];

        for input in inputs {
            // Encrypt the input
            let ciphertext = encrypt_aes128(&key, &iv, input);

            // Decrypt the ciphertext
            let decrypted_text = decrypt_aes128(&key, &iv, &ciphertext);

            // Assert that the decrypted text matches the original input
            assert_eq!(input, decrypted_text, "Decryption failed for input: {}", input);
        }
    }
}

