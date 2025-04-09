use anyhow::{Context, Result};
use eulerst::calc;
use num_bigint::BigUint;
use std::{env, str::FromStr};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        anyhow::bail!(
            "Usage: {} <number> <algorithm: 'euclidean', 'binary', or 'factor'>",
            args.get(0).unwrap_or(&"program".to_string())
        );
    }

    // Parse big integer argument
    let n = BigUint::from_str(&args[1]).context("Failed to parse input number")?;
    let algorithm = args[2].to_lowercase();

    println!("Computing φ({}) using '{}' algorithm:", n, algorithm);

    let phi = match algorithm.as_str() {
        "euclidean" => calc::euler_totient_count(&n, &calc::gcd_ref),
        "binary" => calc::euler_totient_count(&n, &calc::binary_gcd_ref),
        "factor" => {
            // Try to factor and use multiplicative formula
            // NOTE: slow factorization for large n
            let factors = calc::trial_division_factorization(&n);
            calc::euler_totient(&n, Some(&factors))
        }
        _ => anyhow::bail!("Invalid algorithm! Choose 'euclidean', 'binary', or 'factor'."),
    };

    println!("Result: φ({}) = {}", n, phi);

    Ok(())
}
