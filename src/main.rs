use anyhow::{Context, Result};
use eulerst::calc::{binary_gcd, euclidean_gcd, euler_totient};
use num_bigint::BigUint;
use std::env;
use std::str::FromStr;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        anyhow::bail!("Please provide a number and the GCD algorithm (euclidean or binary) as command line arguments.");
    }
    let n = BigUint::from_str(&args[1]).context("Failed to parse the input as a number")?;
    let gcd_algorithm = &args[2];
    let gcd_fn: fn(BigUint, BigUint) -> BigUint = match gcd_algorithm.as_str() {
        "euclidean" => euclidean_gcd,
        "binary" => binary_gcd,
        _ => anyhow::bail!("Invalid GCD algorithm. Please choose either 'euclidean' or 'binary'."),
    };
    let new_n = n.clone();
    println!("φ({}) = {}", n, euler_totient(new_n, gcd_fn));
    Ok(())
}
