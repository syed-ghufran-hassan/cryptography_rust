use rand::Rng;
use sha2::{Digest, Sha256};

/// Represents a zk-SNARK-like system with basic functionality.
mod zk_snark {
    use super::*;

    /// Proving and verification keys.
    pub struct Keys {
        pub proving_key: Vec<u8>,
        pub verification_key: Vec<u8>,
    }

    /// Proof structure.
    pub struct Proof {
        pub proof_data: Vec<u8>,
    }

    /// Generates the proving and verification keys.
    pub fn setup<R: Rng>(rng: &mut R) -> Keys {
        // For simplicity, random bytes represent keys.
        let proving_key = (0..32).map(|_| rng.gen()).collect();
        let verification_key = (0..32).map(|_| rng.gen()).collect();
        Keys {
            proving_key,
            verification_key,
        }
    }

    /// Creates a proof given the proving key, public input, and private witness.
    pub fn prove(proving_key: &[u8], public_input: u32, private_witness: u32) -> Proof {
        let mut hasher = Sha256::new();
        hasher.update(proving_key); // Use proving key in the hash.
        hasher.update(public_input.to_le_bytes());
        hasher.update(private_witness.to_le_bytes());
        let proof_data = hasher.finalize().to_vec();
        Proof { proof_data }
    }

    /// Verifies a proof given the verification key, public input, and proof.
    pub fn verify(verification_key: &[u8], public_input: u32, proof: &Proof) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(verification_key); // Use verification key in the hash.
        hasher.update(public_input.to_le_bytes());
        let expected_proof_data = hasher.finalize();
        proof.proof_data == expected_proof_data.as_slice()
    }
}

fn main() {
    // Example keys (proving and verification keys)
    let mut rng = rand::thread_rng();
    let keys = zk_snark::setup(&mut rng);

    // Example inputs (public and private)
    let public_input = 42;
    let private_witness = 1337;

    // Call prove function to generate proof
    let proof = zk_snark::prove(&keys.proving_key, public_input, private_witness);

    // Call verify function to verify the proof
    let is_valid = zk_snark::verify(&keys.verification_key, public_input, &proof);

    // Output the result of verification
    println!("Verification result: {}", is_valid);
}

#[cfg(test)]
mod tests {
    use super::zk_snark;
    use rand::thread_rng;

    #[test]
    fn test_valid_proof() {
        let mut rng = thread_rng();
        let keys = zk_snark::setup(&mut rng);

        let public_input = 42;
        let private_witness = 1337;

        let proof = zk_snark::prove(&keys.proving_key, public_input, private_witness);
        let is_valid = zk_snark::verify(&keys.verification_key, public_input, &proof);

        // Output the keys and proof for debugging
        println!("Proving Key: {:?}", keys.proving_key);
        println!("Verification Key: {:?}", keys.verification_key);
        println!("Public Input: {}", public_input);
        println!("Private Witness: {}", private_witness);
        println!("Generated Proof Data: {:?}", proof.proof_data);

        assert!(is_valid, "The proof should be valid but isn't.");
    }

    #[test]
    fn test_invalid_proof_with_wrong_input() {
        let mut rng = thread_rng();

        // Setup keys
        let keys = zk_snark::setup(&mut rng);

        // Public input and private witness
        let public_input = 42;
        let private_witness = 1337;

        // Generate proof
        let proof = zk_snark::prove(&keys.proving_key, public_input, private_witness);

        // Verify proof with a different public input
        let is_valid = zk_snark::verify(&keys.verification_key, public_input + 1, &proof);

        // The proof should not be valid
        assert!(!is_valid, "The proof should be invalid but isn't.");
    }

    #[test]
    fn test_invalid_proof_with_wrong_keys() {
        let mut rng = thread_rng();

        // Setup keys
        let keys = zk_snark::setup(&mut rng);

        // Public input and private witness
        let public_input = 42;
        let private_witness = 1337;

        // Generate proof
        let proof = zk_snark::prove(&keys.proving_key, public_input, private_witness);

        // Setup different keys
        let different_keys = zk_snark::setup(&mut rng);

        // Verify proof with different verification key
        let is_valid = zk_snark::verify(&different_keys.verification_key, public_input, &proof);

        // The proof should not be valid
        assert!(!is_valid, "The proof should be invalid with wrong keys but isn't.");
    }
}
