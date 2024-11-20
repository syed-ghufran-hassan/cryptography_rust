# TSS (Threshold Secret Sharing)

A Rust-based implementation of Threshold Secret Sharing (TSS) using Curve25519. This project provides cryptographic tools to split a secret into multiple shares and reconstruct it using a subset of the shares, ensuring that only a threshold of participants can recover the original secret.

## Project Overview

This project implements a cryptographic scheme for sharing a secret among a group of participants, where a threshold number of participants (or shares) are required to reconstruct the original secret. The scheme is based on Curve25519, a high-performance elliptic curve, and can be used in decentralized applications requiring secure key management.

## Features

- Threshold Secret Sharing (TSS): Split a secret into multiple shares and reconstruct it with a subset of those shares.
- Curve25519: Leverages Curve25519 for secure and efficient cryptographic operations.
- Share Generation: Generate shares and combine them back into a secret.
- Secret Reconstruction: Safely reconstruct the secret from the threshold number of shares.

## Implementation example

```rust
use tss::{share_secret, reconstruct_secret};

// Split a secret into shares
let secret = b"my_secret";
let shares = share_secret(secret, 3, 2); // 3 shares, threshold 2

// Reconstruct the secret from shares
let reconstructed_secret = reconstruct_secret(&shares[0..2]);

assert_eq!(secret, reconstructed_secret);
```
