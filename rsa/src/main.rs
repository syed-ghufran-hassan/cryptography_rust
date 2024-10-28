use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;

/// Function to demonstrate RSA encryption and decryption
fn rsa_encrypt_decrypt(plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    // Use a new instance of padding for encryption and decryption
    let encrypted = public_key
        .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), &plaintext[..])
        .expect("encryption failed");
    let decrypted = private_key
        .decrypt(PaddingScheme::new_oaep::<Sha256>(), &encrypted)
        .expect("decryption failed");

    (encrypted, decrypted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsa::errors::Error;

    #[test]
    fn test_encrypt_decrypt_valid_message() {
        let plaintext = b"RSA encryption test";
        let (encrypted, decrypted) = rsa_encrypt_decrypt(plaintext);

        assert_ne!(encrypted, plaintext); // Encrypted text should differ from the plaintext
        assert_eq!(decrypted, plaintext); // Decrypted text should match the original plaintext
    }

    #[test]
    fn test_padding_scheme() {
        let _padding = PaddingScheme::new_oaep::<Sha256>();
        // Test the padding scheme to ensure it can be used without cloning
    }

    #[test]
    fn test_invalid_decryption() {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        let plaintext = b"Different RSA test";

        // Encrypt with a valid public key
        let encrypted = public_key
            .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), &plaintext[..])
            .expect("encryption failed");

        // Use a different private key for decryption to simulate an error
        let other_private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate key");
        let result: Result<Vec<u8>, Error> = other_private_key.decrypt(PaddingScheme::new_oaep::<Sha256>(), &encrypted);

        assert!(result.is_err(), "Expected an error during decryption with incorrect key");
    }

    #[test]
    fn test_empty_plaintext() {
        let plaintext = b"";
        let (_, decrypted) = rsa_encrypt_decrypt(plaintext);

        assert_eq!(decrypted, plaintext); // Decrypted text should match the empty plaintext
    }
}

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
