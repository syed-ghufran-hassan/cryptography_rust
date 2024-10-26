// src/main.rs

use rand::Rng;

const QUARTER_ROUNDS: usize = 20;

pub struct Salsa20 {
    state: [u32; 16],
}

impl Salsa20 {
    pub fn new(key: &[u8; 32], nonce: &[u8; 8]) -> Salsa20 {
        let mut state = [
            0u32, 0u32, 0u32, 0u32,
            0u32, 0u32, 0u32, 0u32,
            0u32, 0u32, 0u32, 0u32,
            0u32, 0u32, 0u32, 0u32,
        ];

        // Load constants
        state[0] = 0x61707865; // "expa"
        state[1] = 0x3320646e; // "nd 3"
        state[2] = 0x79622d32; // "2-by"
        state[3] = 0x6b206574; // "te k"

        // Load key
        for i in 0..8 {
            state[4 + i] = u32::from_le_bytes(key[i * 4..(i + 1) * 4].try_into().unwrap());
        }

        // Load counter
        state[12] = 0; // Counter
        state[13] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        state[14] = u32::from_le_bytes(nonce[4..8].try_into().unwrap()); 

        Salsa20 { state }
    }

    pub fn encrypt(&mut self, input: &mut [u8]) {
        let keystream = self.generate_keystream(input.len());

        for (byte, ks_byte) in input.iter_mut().zip(keystream.iter()) {
            *byte ^= *ks_byte;
        }
    }

    fn generate_keystream(&mut self, length: usize) -> Vec<u8> {
        let mut keystream = Vec::with_capacity(length);
        let mut state = self.state;

        for _ in 0..(length / 64) + 1 {
            // Copy state for output
            let mut output = state;

            // Salsa20 core function
            for _ in 0..QUARTER_ROUNDS {
                self.quarter_round(&mut output);
            }

            // Output generation
            for i in 0..16 {
                let keystream_word = output[i].to_le_bytes();
                keystream.extend_from_slice(&keystream_word);
            }

            // Increment counter
            state[12] = state[12].wrapping_add(1);
        }

        keystream.truncate(length);
        keystream
    }

    fn quarter_round(&self, state: &mut [u32; 16]) {
        // Perform a quarter round on the state
        let x = state;

        x[0] = x[0].wrapping_add(x[4]);
        x[12] = x[12] ^ x[0];
        x[12] = x[12].rotate_left(16);

        x[8] = x[8].wrapping_add(x[12]);
        x[4] = x[4] ^ x[8];
        x[4] = x[4].rotate_left(12);

        x[0] = x[0].wrapping_add(x[4]);
        x[12] = x[12] ^ x[0];
        x[12] = x[12].rotate_left(8);

        x[8] = x[8].wrapping_add(x[12]);
        x[4] = x[4] ^ x[8];
        x[4] = x[4].rotate_left(7);
    }
}

fn main() {
    let key: [u8; 32] = rand::thread_rng().gen();
    let nonce: [u8; 8] = rand::thread_rng().gen();

    let mut salsa = Salsa20::new(&key, &nonce);

    let mut plaintext = b"Hello, Salsa20! This is a test message.".to_vec();

    println!("Original plaintext: {:?}", String::from_utf8_lossy(&plaintext));

    salsa.encrypt(&mut plaintext);
    println!("Encrypted text: {:?}", plaintext);

    // To decrypt, create a new Salsa20 instance with the same key and nonce
    let mut salsa_decrypt = Salsa20::new(&key, &nonce);
    salsa_decrypt.encrypt(&mut plaintext);
    println!("Decrypted text: {:?}", String::from_utf8_lossy(&plaintext));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salsa20_encryption_decryption() {
        let key: [u8; 32] = rand::thread_rng().gen();
        let nonce: [u8; 8] = rand::thread_rng().gen();
        let mut salsa = Salsa20::new(&key, &nonce);

        let mut plaintext = b"Test encryption message".to_vec();
        let original_plaintext = plaintext.clone();

        // Encrypt the plaintext
        salsa.encrypt(&mut plaintext);
        assert_ne!(plaintext, original_plaintext, "Ciphertext should not be equal to plaintext");

        // Decrypt the ciphertext
        let mut salsa_decrypt = Salsa20::new(&key, &nonce);
        salsa_decrypt.encrypt(&mut plaintext);
        assert_eq!(plaintext, original_plaintext, "Decrypted text should match original plaintext");
    }

    #[test]
    fn test_empty_message() {
        let key: [u8; 32] = rand::thread_rng().gen();
        let nonce: [u8; 8] = rand::thread_rng().gen();
        let mut salsa = Salsa20::new(&key, &nonce);

        let mut empty_message: Vec<u8> = vec![];
        let original_message = empty_message.clone();

        salsa.encrypt(&mut empty_message);
        assert_eq!(empty_message, original_message, "Empty message should remain unchanged after encryption/decryption");
    }

    #[test]
    fn test_encrypt_large_message() {
        let key: [u8; 32] = rand::thread_rng().gen();
        let nonce: [u8; 8] = rand::thread_rng().gen();
        let mut salsa = Salsa20::new(&key, &nonce);

        let mut large_message = vec![0u8; 1024]; // 1 KB message
        let original_message = large_message.clone();

        salsa.encrypt(&mut large_message);
        assert_ne!(large_message, original_message, "Ciphertext should not match plaintext for large message");

        let mut salsa_decrypt = Salsa20::new(&key, &nonce);
        salsa_decrypt.encrypt(&mut large_message);
        assert_eq!(large_message, original_message, "Decrypted large message should match original plaintext");
    }
}
