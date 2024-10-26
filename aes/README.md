# AES-128 CBC Mode Encryption and Decryption in Rust

This project demonstrates how to implement AES-128 encryption and decryption using the CBC (Cipher Block Chaining) mode with PKCS7 padding in Rust. AES-128 (Advanced Encryption Standard) is a symmetric encryption algorithm widely used for secure data transmission. The project uses the aes crate for the core AES encryption, block-modes for the CBC mode, and hex-literal to conveniently handle key and IV (Initialization Vector) in hexadecimal format.

## Key Concepts:

- **AES-128:** A symmetric encryption algorithm with a block size of 128 bits (16 bytes). It uses a 128-bit secret key to encrypt and decrypt data.
- **CBC (Cipher Block Chaining):** A mode of operation for block ciphers. It combines each block of plaintext with the previous ciphertext block before encryption, making the encryption process more secure by introducing an IV for the first block.
- **PKCS7 Padding:** A padding scheme that ensures the length of the data being encrypted is a multiple of the block size.

## **Features:**

**Encrypt AES-128 in CBC Mode:**

- The project encrypts a given plaintext using AES-128 with a 16-byte key and IV.
- Uses CBC mode to chain blocks of data.

**Decrypt AES-128 in CBC Mode:**

- The project decrypts a ciphertext back into plaintext using the same AES-128 key and IV.
- Verifies the correct decryption using PKCS7 padding to handle block alignment.

**Hexadecimal Encoding:**

The key and IV are provided in hexadecimal format using the hex_literal crate for easy readability.

## Code Structure:

**encrypt_aes128:** A function that takes a key, IV, and plaintext, and returns the encrypted ciphertext.

**decrypt_aes128:** A function that takes the same key, IV, and ciphertext, and returns the original plaintext by decrypting the data.

**main:** The entry point of the program, where example key, IV, and plaintext are defined, and both encryption and decryption are demonstrated.

When running the program, you should see the encrypted ciphertext and then the decrypted text, verifying that the encryption and decryption process works correctly.
