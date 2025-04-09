//! Euler's Totient and GCD library (BigUint)
//!
//! Provides:
//! - Euclidean & Binary GCD
//! - Euler's totient via counting or prime factorization
//! - RSA totient
//! - Simple trial division factorization

pub mod calc {
    use num_bigint::{BigUint, ToBigUint};
    use num_integer::Integer;
    use num_traits::{One, Zero};
    use std::collections::HashMap;
    use std::time::Instant;

    /// Euclidean gcd (ownership version)
    pub fn euclidean_gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
        while !b.is_zero() {
            let tmp = b.clone();
            b = a % &b;
            a = tmp;
        }
        a
    }

    /// Binary gcd (ownership version)
    pub fn binary_gcd(mut u: BigUint, mut v: BigUint) -> BigUint {
        use std::cmp::min;

        if u.is_zero() {
            return v;
        }
        if v.is_zero() {
            return u;
        }

        let shift = min(
            u.trailing_zeros().unwrap_or(0),
            v.trailing_zeros().unwrap_or(0),
        );

        u >>= u.trailing_zeros().unwrap_or(0);
        v >>= v.trailing_zeros().unwrap_or(0);

        while u != v {
            if u > v {
                u -= &v;
                u >>= u.trailing_zeros().unwrap_or(0);
            } else {
                v -= &u;
                v >>= v.trailing_zeros().unwrap_or(0);
            }
        }

        u << shift
    }

    /// Reference Euclidean gcd
    pub fn gcd_ref(a: &BigUint, b: &BigUint) -> BigUint {
        euclidean_gcd(a.clone(), b.clone())
    }

    /// Reference binary gcd
    pub fn binary_gcd_ref(a: &BigUint, b: &BigUint) -> BigUint {
        binary_gcd(a.clone(), b.clone())
    }

    /// Euler's totient by brute-force counting
    ///
    /// Prints progress every 5%
    pub fn euler_totient_count<F>(n: &BigUint, gcd_fn: &F) -> BigUint
    where
        F: Fn(&BigUint, &BigUint) -> BigUint,
    {
        if n.is_zero() {
            return BigUint::zero();
        }
        if n.is_one() {
            return BigUint::one();
        }

        println!("Calculating φ({}) (count method)...", n);

        let one = BigUint::one();
        let hundred = BigUint::from(100u32);
        let mut coprime_count = BigUint::zero();
        let mut last_percent = 0u32;
        let mut i = BigUint::one();

        let start = Instant::now();

        while &i < n {
            if gcd_fn(&i, n).is_one() {
                coprime_count += &one;
            }

            // Progress report
            let percent = ((&i * &hundred) / n)
                .to_u32_digits()
                .first()
                .copied()
                .unwrap_or(0);
            if percent != last_percent && percent % 5 == 0 {
                let elapsed = start.elapsed();
                println!("Progress: {:3}% elapsed {:?}", percent, elapsed);
                last_percent = percent;
            }

            i += &one;
        }

        coprime_count
    }

    /// RSA totient function φ(p*q)
    pub fn euler_totient_rsa(p: &BigUint, q: &BigUint) -> BigUint {
        assert!(p > &BigUint::one() && q > &BigUint::one());
        (p - 1u32) * (q - 1u32)
    }

    /// Euler's totient using known prime factorization
    ///
    /// Input: map of prime=>exponent, e.g., n = 60 => {2:2, 3:1, 5:1}
    pub fn euler_totient_factors(factors: &HashMap<BigUint, u32>) -> BigUint {
        if factors.is_empty() {
            return BigUint::one();
        }

        let mut result = BigUint::one();

        for (p, k) in factors {
            let p_minus_1 = p - 1u32;
            result *= &p_minus_1 * p.pow(k - 1);
        }

        // Compute n from factors
        let mut n_reconstructed = BigUint::one();
        for (p, k) in factors {
            n_reconstructed *= p.pow(*k);
        }

        result
    }

    /// Naive trial division factorization
    ///
    /// WARNING: Use only for small integers
    pub fn trial_division_factorization(n: &BigUint) -> HashMap<BigUint, u32> {
        let mut n = n.clone();
        let mut factors = HashMap::new();

        let two = 2u32.to_biguint().unwrap();
        let mut d = two.clone();

        while &d * &d <= n {
            let mut count = 0;
            while (&n % &d).is_zero() {
                n /= &d;
                count += 1;
            }
            if count > 0 {
                factors.insert(d.clone(), count);
            }
            d += 1u32;
        }

        if n > BigUint::one() {
            factors.insert(n, 1);
        }

        factors
    }

    /// Adaptive phi(n): uses factorization if given or possible,
    /// else defaults to the coprime count method
    pub fn euler_totient(n: &BigUint, maybe_factors: Option<&HashMap<BigUint, u32>>) -> BigUint {
        if n.is_zero() {
            return BigUint::zero();
        }
        if n.is_one() {
            return BigUint::one();
        }

        let factor_map = match maybe_factors {
            Some(f) => f.clone(),
            None => trial_division_factorization(n),
        };

        euler_totient_factors(&factor_map)
    }

    /// Optional: parallelized counting method (feature-gated)
    #[cfg(feature = "parallel")]
    pub fn euler_totient_count_parallel<F>(n: &BigUint, gcd_fn: &F) -> BigUint
    where
        F: Sync + Fn(&BigUint, &BigUint) -> BigUint,
    {
        use num_traits::cast::ToPrimitive;
        use rayon::prelude::*;

        let one = BigUint::one();
        let n_u64 = n
            .to_u64()
            .expect("Convert n to u64 (only works for moderate size)");

        (1..n_u64)
            .into_par_iter()
            .map(|i| {
                let i_b = BigUint::from(i);
                if gcd_fn(&i_b, n).is_one() {
                    one.clone()
                } else {
                    BigUint::zero()
                }
            })
            .sum()
    }
}
