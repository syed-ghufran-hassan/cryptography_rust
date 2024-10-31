use k256::{
    ecdsa::{SigningKey, Signature, signature::Signer, signature::Verifier, VerifyingKey},
    elliptic_curve::sec1::ToEncodedPoint,
};
use rand_core::OsRng; // Random number generator for secure key generation

fn main() {
    // Generate a random signing key (private key)
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key); // Derive the verifying key (public key) from the signing key

    // Print the public key in uncompressed format
    let public_key_bytes = verifying_key.to_encoded_point(false);
    println!("Public Key: {:?}", public_key_bytes.as_bytes());

    // Message to sign
    let message = b"ECC signing in Rust";

    // Sign the message
    let signature: Signature = signing_key.sign(message);
    println!("Signature: {:?}", signature);

    // Verify the signature
    let is_valid = verifying_key.verify(message, &signature).is_ok();
    println!("Is signature valid? {}", is_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        // Generate a signing key (private key) and derive the verifying key (public key)
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        // Ensure the public key can be serialized
        let public_key_bytes = verifying_key.to_encoded_point(false);
        assert!(!public_key_bytes.as_bytes().is_empty(), "Public key should not be empty");

        println!("Public Key generated successfully");
    }

    #[test]
    fn test_sign_message() {
        // Generate a signing key (private key) and derive the verifying key (public key)
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);

        // Message to sign
        let message = b"Test signing with ECC";

        // Sign the message
        let signature: Signature = signing_key.sign(message);
        
        // Ensure the signature is valid
        let is_valid = verifying_key.verify(message, &signature).is_ok();
        assert!(is_valid, "Signature should be valid");

        println!("Message signed and verified successfully");
    }

    #[test]
    fn test_invalid_signature() {
        // Generate two different signing keys
        let signing_key1 = SigningKey::random(&mut OsRng);
        let _verifying_key1 = VerifyingKey::from(&signing_key1);

        let signing_key2 = SigningKey::random(&mut OsRng);
        
        // Message to sign
        let message = b"Testing invalid signature";

        // Sign the message with the first key
        let signature: Signature = signing_key1.sign(message);

        // Try to verify the signature with a different public key (should fail)
        let is_valid = VerifyingKey::from(&signing_key2).verify(message, &signature).is_ok();
        assert!(!is_valid, "Signature should be invalid with a different public key");

        println!("Invalid signature test passed");
    }
}
