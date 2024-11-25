# A Simplified zk-SNARK Proof System

This project is a basic implementation of a Zero-Knowledge Succinct Non-Interactive Argument of Knowledge (zk-SNARK), which is a cryptographic proof system that allows one party to prove to another that a statement is true, without revealing any information beyond the validity of the statement itself. zk-SNARKs are widely used in blockchain technologies, particularly in privacy-preserving protocols like Zcash.

In this implementation, the system generates cryptographic proofs that can be used to verify the correctness of computations performed by the prover, without revealing sensitive data, such as the private inputs used in the computation.

## Key Features of the Project:

###  Proving and Verification Keys:

The project includes the generation of proving and verification keys. These keys are essential to the zk-SNARK protocol:

-  Proving Key: Used by the prover to create the proof.

- Verification Key: Used by the verifier to check the validity of the proof.

### Proof Generation:

- The prover generates a cryptographic proof using the proving key, public input, and private witness. The private witness is typically a secret value that demonstrates knowledge of a solution to a problem without revealing the solution itself.

### Proof Verification:

- The verifier checks the validity of the proof using the verification key and public input. If the proof is valid, the verifier is assured that the prover knows the solution to a problem without being told what the solution is.

### Test Cases:

- The project includes multiple test cases to verify that the proof system is working as expected:
- Test for Valid Proof: Verifies that the proof is valid when the correct public input and private witness are used.
- Test for Invalid Proof with Wrong Input: Ensures that the proof verification fails when an incorrect public input is provided.
- Test for Invalid Proof with Wrong Keys: Ensures that the proof verification fails when the wrong verification keys are used.

### Hashing Mechanism:

The project uses SHA-256 as the hashing mechanism for creating the proof. This ensures that the proof data is derived from the proving key, public input, and private witness in a secure manner.
