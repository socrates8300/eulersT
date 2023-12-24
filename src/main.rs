use anyhow::{Context, Result};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::env;
use std::str::FromStr;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        anyhow::bail!("Please provide a number as a command line argument.");
    }
    let n = BigUint::from_str(&args[1]).context("Failed to parse the input as a number")?;
    println!("φ({}) = {}", n, euler_totient(&n));
    Ok(())
}

fn euler_totient(n: &BigUint) -> BigUint {
    let mut count = BigUint::zero();
    let mut i = BigUint::one();
    while &i < n {
        if gcd(&i, &n).is_one() {
            count += BigUint::one();
        }
        i += BigUint::one();
    }
    count
}

fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if *b != BigUint::zero() {
        gcd(b, &(a % b))
    } else {
        a.clone()
    }
}
