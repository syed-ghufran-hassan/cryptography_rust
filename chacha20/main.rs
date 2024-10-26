use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use chacha20::{ChaCha20, Key, Nonce, cipher::{KeyIvInit, StreamCipher}};

fn encrypt(data: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> Vec<u8> {
    let key = Key::from_slice(key);
    let nonce = Nonce::from_slice(nonce);
    let mut cipher = ChaCha20::new(key, nonce);
    let mut encrypted_data = data.to_vec();
    cipher.apply_keystream(&mut encrypted_data);
    encrypted_data
}

fn decrypt(encrypted_data: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> Vec<u8> {
    let key = Key::from_slice(key);
    let nonce = Nonce::from_slice(nonce);
    let mut cipher = ChaCha20::new(key, nonce);
    let mut decrypted_data = encrypted_data.to_vec();
    cipher.apply_keystream(&mut decrypted_data);
    decrypted_data
}

fn main() {
    // Generate a random key and nonce
    let mut rng = ChaCha20Rng::from_entropy();
    let mut key = [0u8; 32]; // 256-bit key for ChaCha20
    let mut nonce = [0u8; 12]; // 96-bit nonce for ChaCha20

    rng.fill_bytes(&mut key); // Fill key with random bytes
    rng.fill_bytes(&mut nonce); // Fill nonce with random bytes

    // Data to encrypt
    let data = b"Hello, world!";
    
    // Encrypt the data
    let encrypted_data = encrypt(data, &key, &nonce);

    // Output the encrypted data
    println!("Encrypted data: {:?}", encrypted_data);

    // Decrypt the data
    let decrypted_data = decrypt(&encrypted_data, &key, &nonce);

    // Output the decrypted data
    println!("Decrypted data: {:?}", String::from_utf8(decrypted_data).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut key = [0u8; 32]; // 256-bit key for ChaCha20
        let mut nonce = [0u8; 12]; // 96-bit nonce for ChaCha20

        rng.fill_bytes(&mut key); // Fill key with random bytes
        rng.fill_bytes(&mut nonce); // Fill nonce with random bytes

        let data = b"Test encryption and decryption";

        // Encrypt the data
        let encrypted_data = encrypt(data, &key, &nonce);
        
        // Ensure that encrypted data is not equal to original data
        assert_ne!(encrypted_data, data);

        // Decrypt the data
        let decrypted_data = decrypt(&encrypted_data, &key, &nonce);

        // Ensure that decrypted data matches the original data
        assert_eq!(decrypted_data, data);
    }

    #[test]
    fn test_different_keys() {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut key1 = [0u8; 32];
        let mut key2 = [0u8; 32];
        let mut nonce = [0u8; 12];

        rng.fill_bytes(&mut key1); // Fill key1 with random bytes
        rng.fill_bytes(&mut key2); // Fill key2 with random bytes
        rng.fill_bytes(&mut nonce); // Fill nonce with random bytes

        let data = b"Test different keys";

        // Encrypt the data with the first key
        let encrypted_data1 = encrypt(data, &key1, &nonce);
        
        // Encrypt the data with the second key
        let encrypted_data2 = encrypt(data, &key2, &nonce);

        // Ensure that the encrypted outputs are different
        assert_ne!(encrypted_data1, encrypted_data2);
    }

    #[test]
    fn test_same_key_same_nonce() {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut key = [0u8; 32];
        let mut nonce = [0u8; 12];

        rng.fill_bytes(&mut key); // Fill key with random bytes
        rng.fill_bytes(&mut nonce); // Fill nonce with random bytes

        let data = b"Test same key and nonce";

        // Encrypt the data
        let encrypted_data = encrypt(data, &key, &nonce);
        
        // Decrypt the data
        let decrypted_data1 = decrypt(&encrypted_data, &key, &nonce);
        let decrypted_data2 = decrypt(&encrypted_data, &key, &nonce); // Decrypt again

        // Ensure that both decrypted outputs match the original data
        assert_eq!(decrypted_data1, data);
        assert_eq!(decrypted_data2, data);
    }
}
