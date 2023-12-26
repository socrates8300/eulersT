pub mod calc {

    use num_bigint::BigUint;
    use num_traits::{One, Zero};

    pub fn euler_totient_rsa(p: &BigUint, q: &BigUint) -> BigUint {
        // Since p and q are prime, φ(p) = p - 1 and φ(q) = q - 1
        // Therefore, φ(N) = φ(p) * φ(q) = (p - 1) * (q - 1)
        let one = BigUint::one();
        let p_minus_one = p - &one;
        let q_minus_one = q - &one;

        p_minus_one * q_minus_one
    }

    pub fn euler_totient(n: BigUint, gcd_fn: fn(BigUint, BigUint) -> BigUint) -> BigUint {
        let mut count = BigUint::zero();
        let mut i = BigUint::one();
        let hundred = BigUint::from(100u32);
        let progress = BigUint::zero();
        let mut last_percentage = BigUint::zero();

        while i < n {
            if gcd_fn(i.clone(), n.clone()).is_one() {
                count += BigUint::one();
            }
            i += BigUint::one();

            let new_progress = (&i * &hundred) / &n;
            if new_progress > progress {
                let progress = new_progress;
                let percentage = &progress.to_string();
                if percentage != &last_percentage.to_string() {
                    println!("Progress: {}%", percentage);
                    last_percentage = progress;
                }
            }
        }
        count
    }

    pub fn euclidean_gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
        while b != BigUint::zero() {
            let temp_b = b.clone();
            b = a % &b;
            a = temp_b;
        }
        a
    }

    pub fn binary_gcd(mut u: BigUint, mut v: BigUint) -> BigUint {
        if u.is_zero() {
            return v;
        }
        if v.is_zero() {
            return u;
        }

        let shift_u = u.trailing_zeros().unwrap_or(0);
        let shift_v = v.trailing_zeros().unwrap_or(0);
        let shift = std::cmp::min(shift_u, shift_v);

        u >>= shift_u;
        v >>= shift_v;

        while !u.is_zero() {
            u >>= u.trailing_zeros().unwrap_or(0);

            if u >= v {
                std::mem::swap(&mut u, &mut v);
            }

            v -= &u;
        }

        v << shift
    }
}
