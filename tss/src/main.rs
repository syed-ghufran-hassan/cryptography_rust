use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use rand::RngCore;

fn main() {
    let mut rng = OsRng;
    let mut coefficients = Vec::new();

    // Generate random coefficients
    for _ in 0..5 {
        let mut random_bytes = [0u8; 32];
        rng.fill_bytes(&mut random_bytes);
        let random_scalar = Scalar::from_bytes_mod_order(random_bytes);
        coefficients.push(random_scalar);
    }

    // Compute share values
    let mut shares = Vec::new();
    for x in 1..=5 {
        let mut share_value = Scalar::from(0u64);
        let mut power = Scalar::from(1u64);
        for (i, coeff) in coefficients.iter().enumerate() {
            share_value += coeff * power;
            power *= Scalar::from(x as u64);
        }
        shares.push((x, share_value));
    }

    // Reconstruction example (dummy logic for simplicity)
    let mut secret = Scalar::from(0u64);
    for (x_i, y_i) in &shares {
        let mut lagrange_coeff = Scalar::from(1u64);
        for (x_j, _) in &shares {
            if x_i != x_j {
                let numerator = Scalar::from((*x_j as u64));
                let denominator = Scalar::from(((*x_i as i64) - (*x_j as i64)).abs() as u64);

                lagrange_coeff *= numerator * denominator.invert();
            }
        }
        secret += lagrange_coeff * y_i;
    }

    println!("Original Secret: {:?}", coefficients[0]);
    println!("Reconstructed Secret: {:?}", secret);
}
