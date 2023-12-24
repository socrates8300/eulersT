use anyhow::{Context, Result};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::env;
use std::str::FromStr;

// Entry point for the program
fn main() -> Result<()> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    // Check that the correct number of arguments have been provided
    if args.len() <= 2 {
        anyhow::bail!("Please provide a number and the GCD algorithm (euclidean or binary) as command line arguments.");
    }
    // Parse the first argument as a BigUint
    let n = BigUint::from_str(&args[1]).context("Failed to parse the input as a number")?;
    // Get the gcd algorithm to use from the second argument
    let gcd_algorithm = &args[2];
    // Assign the appropriate gcd function based on the gcd_algorithm argument
    let gcd_fn: fn(&BigUint, &BigUint) -> BigUint = match gcd_algorithm.as_str() {
        "euclidean" => euclidean_gcd,
        "binary" => binary_gcd,
        _ => anyhow::bail!("Invalid GCD algorithm. Please choose either 'euclidean' or 'binary'."),
    };
    // Calculate and print the Euler's Totient function of n using the selected gcd function
    println!("φ({}) = {}", n, euler_totient(&n, gcd_fn));
    Ok(())
}

// Function to calculate the Euler's Totient function of n
// TODO: Optimize this function for large n
fn euler_totient(n: &BigUint, gcd_fn: fn(&BigUint, &BigUint) -> BigUint) -> BigUint {
    let mut count = BigUint::zero();
    let mut i = BigUint::one();
    let hundred = BigUint::from(100u32);
    let mut progress = BigUint::zero();
    let mut last_percentage = BigUint::zero();

    while &i < n {
        if gcd_fn(&i, n).is_one() {
            count += BigUint::one();
        }
        i += BigUint::one();

        // Calculate and print the progress
        let new_progress = (&i * &hundred) / n;
        if new_progress > progress {
            progress = new_progress;
            let percentage = &progress.to_string();
            if percentage != &last_percentage.to_string() {
                println!("Progress: {}%", percentage);
                last_percentage = progress.clone();
            }
        }
    }
    count
}


// Function to calculate the greatest common divisor of a and b using the Euclidean algorithm
// TODO: Consider replacing recursion with iteration to avoid potential stack overflow for large inputs
fn euclidean_gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if *b != BigUint::zero() {
        euclidean_gcd(b, &(a % b))
    } else {
        a.clone()
    }
}

// Function to calculate the greatest common divisor of u and v using the binary GCD algorithm (Stein's algorithm)
// TODO: Consider optimizing this function for large inputs
fn binary_gcd(u: &BigUint, v: &BigUint) -> BigUint {
    // let zero = BigUint::zero();
    let mut u = u.clone();
    let mut v = v.clone();

    if u.is_zero() { return v; }
    if v.is_zero() { return u; }

    let shift = (&u | &v).trailing_zeros().unwrap_or(0);

    u >>= u.trailing_zeros().unwrap_or(0);
    v >>= v.trailing_zeros().unwrap_or(0);

    while !u.is_zero() {
        u >>= u.trailing_zeros().unwrap_or(0);

        if u >= v {
            std::mem::swap(&mut u, &mut v);
            u -= &v;
        }

        v >>= v.trailing_zeros().unwrap_or(0);
    }

    v << shift
}
