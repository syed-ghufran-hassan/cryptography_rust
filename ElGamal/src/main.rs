use rand::Rng;

pub struct ElGamal {
    pub p: u64,
    pub g: u64,
    pub private_key: u64,
    pub public_key: u64,
}

impl ElGamal {
    pub fn new(p: u64, g: u64, private_key: u64) -> Self {
        let public_key = g.pow(private_key as u32) % p;
        ElGamal { p, g, private_key, public_key }
    }

    pub fn encrypt(&self, message: u64, k: u64) -> (u64, u64) {
        let c1 = self.g.pow(k as u32) % self.p;
        let c2 = (message * self.public_key.pow(k as u32)) % self.p;
        (c1, c2)
    }

    pub fn decrypt(&self, ciphertext: (u64, u64)) -> u64 {
        let (c1, c2) = ciphertext;
        let s = c1.pow(self.private_key as u32) % self.p;
        let s_inv = mod_inv(s, self.p);
        (c2 * s_inv) % self.p
    }
}

// Helper function to calculate modular inverse
fn mod_inv(a: u64, p: u64) -> u64 {
    let mut t = 0i64; // Use signed integers to handle negative values
    let mut new_t = 1i64;
    let mut r = p as i64;
    let mut new_r = a as i64;

    while new_r != 0 {
        let quotient = r / new_r;

        let temp_t = t;
        t = new_t;
        new_t = temp_t - quotient * new_t;

        let temp_r = r;
        r = new_r;
        new_r = temp_r - quotient * new_r;
    }

    if r > 1 {
        panic!("{} has no modular inverse under modulo {}", a, p);
    }

    // Ensure positive result in the modulo space
    ((t + p as i64) % p as i64) as u64
}

fn main() {
    let elgamal = ElGamal::new(23, 5, 6);
    let message = 12;
    let k = 3; // Randomly chosen ephemeral key
    let ciphertext = elgamal.encrypt(message, k);
    let decrypted_message = elgamal.decrypt(ciphertext);

    println!("ElGamal Encryption: c1 = {}, c2 = {}", ciphertext.0, ciphertext.1);
    println!("ElGamal Decryption: message = {}", decrypted_message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let p = 23;
        let g = 5;
        let private_key = 6;
        let elgamal = ElGamal::new(p, g, private_key);

        let message = 12;
        let k = 3;
        let ciphertext = elgamal.encrypt(message, k);
        let decrypted_message = elgamal.decrypt(ciphertext);

        assert_eq!(decrypted_message, message, "Decrypted message should")
    }
}
